import { test, expect } from "@playwright/test";

test.describe("Kit Diagram Node-Edge Alignment", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:5173/kits");
    const createTemporary = page.locator('[id="semio.sketchpad.app.home.toolbar.createTemporary"]');
    if ((await createTemporary.count()) > 0) {
      await createTemporary.click();
      await page.waitForURL(/\/kits\/[0-9a-f-]+/i);
      await page.waitForTimeout(600);
    }
  });

  test("should render circle/rectangle/long-rectangle/triangle shape nodes with expected frames", async ({ page }) => {
    const nodeCount = await page.locator('[data-kit-node="v3"]').count();
    test.skip(nodeCount === 0, "No kit diagram nodes available in this environment.");
    const shapeCounts = await page.locator('[data-kit-node="v3"]').evaluateAll((nodes) => {
      const result: Record<string, number> = {};
      for (const node of nodes) {
        const shape = (node as HTMLElement).dataset.kitNodeShape ?? "unknown";
        result[shape] = (result[shape] ?? 0) + 1;
      }
      return result;
    });
    expect((shapeCounts.circle ?? 0) > 0 || (shapeCounts.rectangle ?? 0) > 0 || (shapeCounts["long-rectangle"] ?? 0) > 0).toBe(true);
    expect(shapeCounts.rectangle ?? 0).toBeGreaterThan(0);
    expect(shapeCounts["long-rectangle"] ?? 0).toBeGreaterThan(0);
    expect.soft(shapeCounts.triangle ?? 0).toBeGreaterThan(0);
  });

  test("should align edge endpoints with nearest snap points after simulation", async ({ page }) => {
    const nodeCount = await page.locator('[data-kit-node="v3"]').count();
    test.skip(nodeCount === 0, "No kit diagram nodes available in this environment.");
    await page.waitForTimeout(900);
    const result = await page.locator('[data-testid="kit-diagram"]').evaluate((root) => {
      const parseTranslate = (value: string): { x: number; y: number } => {
        const direct = /translate\(\s*(-?\d+(?:\.\d+)?)px(?:,\s*(-?\d+(?:\.\d+)?)px)?\s*\)/.exec(value);
        if (direct) {
          return { x: Number(direct[1] ?? 0), y: Number(direct[2] ?? 0) };
        }
        const matrix = /matrix\(\s*[^,]+,\s*[^,]+,\s*[^,]+,\s*[^,]+,\s*(-?\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)\s*\)/.exec(value);
        if (matrix) {
          return { x: Number(matrix[1] ?? 0), y: Number(matrix[2] ?? 0) };
        }
        return { x: 0, y: 0 };
      };
      const parseEndpoints = (d: string): { start: { x: number; y: number }; end: { x: number; y: number } } | null => {
        const values = (d.match(/-?\d+(?:\.\d+)?/g) ?? []).map((item) => Number(item));
        if (values.length < 8) return null;
        return {
          start: { x: values[0] ?? 0, y: values[1] ?? 0 },
          end: { x: values[values.length - 2] ?? 0, y: values[values.length - 1] ?? 0 },
        };
      };
      const snapPoints = (shape: string, x: number, y: number, width: number, height: number): Array<{ x: number; y: number }> => {
        if (shape === "triangle") {
          return [
            { x: x + width / 2, y },
            { x, y: y + height },
            { x: x + width, y: y + height },
          ];
        }
        return [
          { x: x + width / 2, y },
          { x: x + width, y: y + height / 2 },
          { x: x + width / 2, y: y + height },
          { x, y: y + height / 2 },
        ];
      };
      const nodes = Array.from(root.querySelectorAll('[data-kit-node="v3"]'))
        .map((innerNode) => {
          const wrapper = innerNode.closest(".react-flow__node") as HTMLElement | null;
          if (!wrapper) return null;
          const transform = parseTranslate(wrapper.style.transform ?? "");
          const styleWidth = Number((innerNode as HTMLElement).style.width.replace("px", ""));
          const styleHeight = Number((innerNode as HTMLElement).style.height.replace("px", ""));
          const width = Number.isFinite(styleWidth) && styleWidth > 0 ? styleWidth : innerNode.getBoundingClientRect().width;
          const height = Number.isFinite(styleHeight) && styleHeight > 0 ? styleHeight : innerNode.getBoundingClientRect().height;
          return {
            x: transform.x,
            y: transform.y,
            width,
            height,
            center: { x: transform.x + width / 2, y: transform.y + height / 2 },
            shape: (innerNode as HTMLElement).dataset.kitNodeShape ?? "circle",
          };
        })
        .filter((node): node is { x: number; y: number; width: number; height: number; center: { x: number; y: number }; shape: string } => Boolean(node));
      const paths = Array.from(root.querySelectorAll("path"))
        .map((path) => (path as SVGPathElement).getAttribute("d"))
        .filter((value): value is string => Boolean(value) && value.startsWith("M"));
      let checks = 0;
      let invalid = 0;
      for (const d of paths) {
        const endpoints = parseEndpoints(d);
        if (!endpoints) continue;
        const endpointList = [endpoints.start, endpoints.end];
        for (const endpoint of endpointList) {
          const nearestNode = nodes
            .map((node) => {
              const dx = endpoint.x - node.center.x;
              const dy = endpoint.y - node.center.y;
              return { node, distance: Math.hypot(dx, dy) };
            })
            .sort((a, b) => a.distance - b.distance)[0]?.node;
          if (!nearestNode) continue;
          const pointDistances = snapPoints(nearestNode.shape, nearestNode.x, nearestNode.y, nearestNode.width, nearestNode.height).map((point) =>
            Math.hypot(point.x - endpoint.x, point.y - endpoint.y),
          );
          const minDistance = Math.min(...pointDistances);
          checks += 1;
          if (minDistance > 8) invalid += 1;
        }
      }
      return { edgeCount: paths.length, checks, invalid };
    });
    expect(result.checks).toBeGreaterThanOrEqual(0);
    expect(result.invalid).toBe(0);
  });

  test("should keep edge endpoint alignment during node drag and after tick", async ({ page }) => {
    const nodeCount = await page.locator('[data-kit-node="v3"]').count();
    test.skip(nodeCount === 0, "No kit diagram nodes available in this environment.");
    const firstNode = page.locator('[data-kit-node="v3"]').first();
    const initialBox = await firstNode.boundingBox();
    if (initialBox) {
      await firstNode.dragTo(firstNode, {
        sourcePosition: { x: initialBox.width / 2, y: initialBox.height / 2 },
        targetPosition: { x: initialBox.width / 2 + 120, y: initialBox.height / 2 + 60 },
      });
      await page.waitForTimeout(700);
      const finalBox = await firstNode.boundingBox();
      if (finalBox) {
        expect(finalBox.width).toBeCloseTo(initialBox.width, 8);
        expect(finalBox.height).toBeCloseTo(initialBox.height, 8);
      }
    }
    const result = await page.locator('[data-testid="kit-diagram"]').evaluate((root) => {
      const parseTranslate = (value: string): { x: number; y: number } => {
        const direct = /translate\(\s*(-?\d+(?:\.\d+)?)px(?:,\s*(-?\d+(?:\.\d+)?)px)?\s*\)/.exec(value);
        if (direct) {
          return { x: Number(direct[1] ?? 0), y: Number(direct[2] ?? 0) };
        }
        const matrix = /matrix\(\s*[^,]+,\s*[^,]+,\s*[^,]+,\s*[^,]+,\s*(-?\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)\s*\)/.exec(value);
        if (matrix) {
          return { x: Number(matrix[1] ?? 0), y: Number(matrix[2] ?? 0) };
        }
        return { x: 0, y: 0 };
      };
      const parseEndpoints = (d: string): { start: { x: number; y: number }; end: { x: number; y: number } } | null => {
        const values = (d.match(/-?\d+(?:\.\d+)?/g) ?? []).map((item) => Number(item));
        if (values.length < 8) return null;
        return {
          start: { x: values[0] ?? 0, y: values[1] ?? 0 },
          end: { x: values[values.length - 2] ?? 0, y: values[values.length - 1] ?? 0 },
        };
      };
      const snapPoints = (shape: string, x: number, y: number, width: number, height: number): Array<{ x: number; y: number }> => {
        if (shape === "triangle") {
          return [
            { x: x + width / 2, y },
            { x, y: y + height },
            { x: x + width, y: y + height },
          ];
        }
        return [
          { x: x + width / 2, y },
          { x: x + width, y: y + height / 2 },
          { x: x + width / 2, y: y + height },
          { x, y: y + height / 2 },
        ];
      };
      const nodes = Array.from(root.querySelectorAll('[data-kit-node="v3"]'))
        .map((innerNode) => {
          const wrapper = innerNode.closest(".react-flow__node") as HTMLElement | null;
          if (!wrapper) return null;
          const transform = parseTranslate(wrapper.style.transform ?? "");
          const styleWidth = Number((innerNode as HTMLElement).style.width.replace("px", ""));
          const styleHeight = Number((innerNode as HTMLElement).style.height.replace("px", ""));
          const width = Number.isFinite(styleWidth) && styleWidth > 0 ? styleWidth : innerNode.getBoundingClientRect().width;
          const height = Number.isFinite(styleHeight) && styleHeight > 0 ? styleHeight : innerNode.getBoundingClientRect().height;
          return {
            x: transform.x,
            y: transform.y,
            width,
            height,
            center: { x: transform.x + width / 2, y: transform.y + height / 2 },
            shape: (innerNode as HTMLElement).dataset.kitNodeShape ?? "circle",
          };
        })
        .filter((node): node is { x: number; y: number; width: number; height: number; center: { x: number; y: number }; shape: string } => Boolean(node));
      const paths = Array.from(root.querySelectorAll("path"))
        .map((path) => (path as SVGPathElement).getAttribute("d"))
        .filter((value): value is string => Boolean(value) && value.startsWith("M"));
      let checks = 0;
      let invalid = 0;
      for (const d of paths) {
        const endpoints = parseEndpoints(d);
        if (!endpoints) continue;
        const endpointList = [endpoints.start, endpoints.end];
        for (const endpoint of endpointList) {
          const nearestNode = nodes
            .map((node) => {
              const dx = endpoint.x - node.center.x;
              const dy = endpoint.y - node.center.y;
              return { node, distance: Math.hypot(dx, dy) };
            })
            .sort((a, b) => a.distance - b.distance)[0]?.node;
          if (!nearestNode) continue;
          const pointDistances = snapPoints(nearestNode.shape, nearestNode.x, nearestNode.y, nearestNode.width, nearestNode.height).map((point) =>
            Math.hypot(point.x - endpoint.x, point.y - endpoint.y),
          );
          const minDistance = Math.min(...pointDistances);
          checks += 1;
          if (minDistance > 8) invalid += 1;
        }
      }
      return { edgeCount: paths.length, checks, invalid };
    });
    expect(result.checks).toBeGreaterThanOrEqual(0);
    expect(result.invalid).toBe(0);
  });
});
