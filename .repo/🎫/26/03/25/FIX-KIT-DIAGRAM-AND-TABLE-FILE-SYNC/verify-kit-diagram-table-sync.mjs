import { chromium } from "playwright";
import { readFile } from "node:fs/promises";

const baseUrl = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4173";
const kitFixturePath = "/workspaces/semio/semio/assets/semio/kit_metabolism.json";

const setTogglePressed = async (toggle, pressed) => {
  const currentState = (await toggle.getAttribute("aria-pressed").catch(() => null)) ?? (await toggle.getAttribute("data-state").catch(() => null));
  const isPressed = currentState === "true" || currentState === "on";
  if (isPressed !== pressed) {
    await toggle.click({ force: true });
  }
};

const main = async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ baseURL: baseUrl });

  page.on("console", (message) => {
    const text = message.text();
    if (text.includes("[DEBUG]")) {
      console.log(text);
    }
  });

  const kitFixture = JSON.parse(await readFile(kitFixturePath, "utf8"));

  try {
    await page.goto("/");
    await page.waitForFunction(() => Boolean(window.__SEMIO_STORE__), undefined, { timeout: 30000 });

    const kitGuid = await page.evaluate(async (fixture) => {
      const store = window.__SEMIO_STORE__;
      const existing = (store.kitShallows?.() ?? []).find((kit) => String(kit?.name ?? "").toLowerCase().includes("metabolism"));
      if (existing?.guid) {
        return existing.guid;
      }
      await store.execute("semio.sketchpad.createKit", "semio.sketchpad.ticket.verifyKitDiagramTableSync", fixture, false, false);
      const created = (store.kitShallows?.() ?? []).find((kit) => String(kit?.name ?? "").toLowerCase().includes("metabolism"));
      return created?.guid ?? null;
    }, kitFixture);

    if (!kitGuid) {
      throw new Error("Unable to create or locate the metabolism kit in the sketchpad store");
    }

    await page.goto(`/kits/${kitGuid}`);
    await page.locator('tbody tr[data-row-id]').first().waitFor({ timeout: 30000 });
    await page.locator('[data-testid="kit-diagram"]').waitFor({ timeout: 30000 });

    const filesToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showFiles"]');
    const foldersToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showFolders"]');

    if ((await filesToggle.count()) > 0) {
      await setTogglePressed(filesToggle, true);
    }
    if ((await foldersToggle.count()) > 0) {
      await setTogglePressed(foldersToggle, true);
    }

    await page.waitForTimeout(1000);

    const referencedCounts = await page.evaluate(() => {
      const normalizePath = (path) => path.replace(/^\.?\//, "").replace(/\/+$/, "");
      const store = window.__SEMIO_STORE__;
      const actor = window.__SEMIO_ACTOR__;
      const kitGuid = window.location.pathname.match(/\/kits\/([^/]+)/)?.[1];
      if (!store || !kitGuid || !store.hasKit(kitGuid)) {
        return null;
      }

      const kitStore = store.kit(kitGuid);
      const kit = kitStore.snapshot();
      const foldersByGuid = new Map((kit.folders ?? []).filter(Boolean).map((folder) => [folder.guid, folder]));
      const folderPathByGuid = new Map();

      const getFolderPath = (folderGuid) => {
        if (!folderGuid) return "";
        if (folderPathByGuid.has(folderGuid)) return folderPathByGuid.get(folderGuid);
        const folder = foldersByGuid.get(folderGuid);
        if (!folder) {
          folderPathByGuid.set(folderGuid, "");
          return "";
        }
        const parentPath = getFolderPath(folder.parent?.guid);
        const folderPath = normalizePath(parentPath ? `${parentPath}/${folder.name}` : folder.name);
        folderPathByGuid.set(folderGuid, folderPath);
        return folderPath;
      };

      const referencedFilePaths = new Set();
      Array.from(kitStore.fileUrls.keys()).forEach((path) => {
        const normalizedPath = normalizePath(path);
        if (normalizedPath) {
          referencedFilePaths.add(normalizedPath);
        }
      });

      const referencedFolderPaths = new Set();
      referencedFilePaths.forEach((filePath) => {
        const segments = filePath.split("/").filter(Boolean);
        for (let index = 1; index < segments.length; index += 1) {
          referencedFolderPaths.add(segments.slice(0, index).join("/"));
        }
      });

      const referencedFiles = (kit.files ?? []).filter((file) => {
        if (!file?.guid || foldersByGuid.has(file.guid)) return false;
        const parentPath = getFolderPath(file.folder?.guid);
        const storagePath = normalizePath(parentPath ? `${parentPath}/${file.name}` : file.name);
        if (!storagePath || referencedFolderPaths.has(storagePath)) return false;
        return referencedFilePaths.has(storagePath);
      });

      const foldersWithFileDescendants = new Set();
      referencedFiles.forEach((file) => {
        let currentFolderGuid = file.folder?.guid;
        while (currentFolderGuid) {
          if (foldersWithFileDescendants.has(currentFolderGuid)) break;
          foldersWithFileDescendants.add(currentFolderGuid);
          currentFolderGuid = foldersByGuid.get(currentFolderGuid)?.parent?.guid;
        }
      });

      const referencedFolders = (kit.folders ?? []).filter((folder) => {
        const folderPath = getFolderPath(folder.guid);
        return (folderPath && referencedFolderPaths.has(folderPath)) || foldersWithFileDescendants.has(folder.guid);
      });

      actor?.send({
        type: "KIT.SET_EXPANDED_ROWS",
        kitGuid,
        expandedRows: new Set(referencedFolders.map((folder) => `folder-${folder.guid}`)),
      });

      return {
        referencedFiles: referencedFiles.length,
        referencedFolders: referencedFolders.length,
      };
    });

    if (!referencedCounts) {
      throw new Error("Unable to resolve referenced kit artifact counts from the running app");
    }

    await page.waitForTimeout(500);

    const observedCounts = {
      tableFiles: await page.locator('tr[data-row-id^="file-"]').count(),
      tableFolders: await page.locator('tr[data-row-id^="folder-"]').count(),
      diagramFiles: await page.locator('[data-kit-node-kind="file"]').count(),
      diagramFolders: await page.locator('[data-kit-node-kind="folder"]').count(),
    };

    const counts = { ...referencedCounts, ...observedCounts };
    console.log(`[DEBUG] kit diagram/table sync counts ${JSON.stringify(counts)}`);

    if (counts.referencedFiles <= 0) {
      throw new Error(`Expected referenced files to be present, got ${counts.referencedFiles}`);
    }
    if (counts.tableFiles !== counts.referencedFiles) {
      throw new Error(`Table file rows (${counts.tableFiles}) do not match referenced files (${counts.referencedFiles})`);
    }
    if (counts.diagramFiles !== counts.referencedFiles) {
      throw new Error(`Diagram file nodes (${counts.diagramFiles}) do not match referenced files (${counts.referencedFiles})`);
    }
    if (counts.tableFolders !== counts.referencedFolders) {
      throw new Error(`Table folder rows (${counts.tableFolders}) do not match referenced folders (${counts.referencedFolders})`);
    }
    if (counts.diagramFolders !== counts.referencedFolders) {
      throw new Error(`Diagram folder nodes (${counts.diagramFolders}) do not match referenced folders (${counts.referencedFolders})`);
    }
  } finally {
    await page.close().catch(() => {});
    await browser.close().catch(() => {});
  }
};

await main();
