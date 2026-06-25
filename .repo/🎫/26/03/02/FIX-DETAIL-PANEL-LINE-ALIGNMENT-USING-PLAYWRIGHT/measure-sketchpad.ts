import { chromium } from "playwright";
async function measure() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1920, height: 1080 } });
  await page.goto("http://localhost:5173/");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(3000);
  const zipPath = "/workspaces/semio/assets/compose/metabolism.zip";
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: "attached", timeout: 10000 });
  const [fileChooser] = await Promise.all([
    page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
    fileInput.dispatchEvent("click")
  ]);
  if (fileChooser) await fileChooser.setFiles(zipPath);
  else { await fileInput.setInputFiles(zipPath); await fileInput.evaluate((el) => { el.dispatchEvent(new Event("change", { bubbles: true })); }); }
  const metabolismText = page.getByText("Metabolism", { exact: true }).first();
  await metabolismText.waitFor({ state: "visible", timeout: 60000 });
  await page.waitForTimeout(500);
  const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
  const isTableRowVisible = await tableRow.isVisible().catch(() => false);
  if (isTableRowVisible) await tableRow.dblclick({ force: true });
  else await page.getByText("Metabolism").first().dblclick({ force: true });
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(3000);
  console.log("[DEBUG] URL:", page.url());
  let sectionCount = await page.locator('[data-slot="tree-section-row"]').count();
  let itemCount = await page.locator('[data-slot="tree-item-row"]').count();
  console.log("[DEBUG] Before panel toggle: sections=" + sectionCount + " items=" + itemCount);
  if (sectionCount === 0 && itemCount === 0) {
    console.log("[DEBUG] Trying to look for right panel toggle...");
    const rightToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const toggleExists = await rightToggle.count();
    console.log("[DEBUG] Right panel toggle count:", toggleExists);
    if (toggleExists > 0) {
      await rightToggle.click();
      await page.waitForTimeout(2000);
    }
  }
  sectionCount = await page.locator('[data-slot="tree-section-row"]').count();
  itemCount = await page.locator('[data-slot="tree-item-row"]').count();
  console.log("[DEBUG] After panel toggle: sections=" + sectionCount + " items=" + itemCount);
  if (sectionCount === 0 && itemCount === 0) {
    console.log("[DEBUG] Dumping all data-slot values on page...");
    const slots = await page.evaluate(() => {
      return Array.from(document.querySelectorAll("[data-slot]")).map(el => ({
        slot: el.getAttribute("data-slot"),
        id: el.id,
        tag: el.tagName,
        visible: el.getBoundingClientRect().width > 0
      }));
    });
    for (const s of slots.slice(0, 30)) {
      console.log(`  slot="${s.slot}" id="${s.id}" tag=${s.tag} visible=${s.visible}`);
    }
    const panelDivs = await page.evaluate(() => {
      return Array.from(document.querySelectorAll("[class*='side-panel'], [class*='panel'], [role='tree']")).map(el => ({
        cls: el.className.substring(0, 100),
        tag: el.tagName,
        children: el.children.length
      })).slice(0, 20);
    });
    console.log("[DEBUG] Panel-related elements:", JSON.stringify(panelDivs, null, 2));
  }
  const measurements = await page.evaluate(() => {
    const results: any[] = [];
    document.querySelectorAll('[data-slot="tree-section-row"]').forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const label = row.querySelector('[data-slot="tree-label"]');
      const svgs = row.querySelectorAll("svg");
      const chevronSvg = Array.from(svgs).find(svg => {
        const cls = svg.getAttribute("class") || "";
        return cls.includes("lucide-chevron") || cls.includes("size-");
      });
      if (chevronSvg) {
        const r = chevronSvg.getBoundingClientRect();
        results.push({ type: "Section", label: label?.textContent?.trim(), padL: window.getComputedStyle(row).paddingLeft, chevCenter: r.left + r.width / 2 - rowRect.left, chevW: r.width });
      }
    });
    document.querySelectorAll('[data-slot="tree-item-row"]').forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const label = row.querySelector('[data-slot="tree-label"]');
      let ci: any = null;
      row.querySelectorAll("button").forEach(btn => {
        const svg = btn.querySelector("svg");
        if (svg) { const r = svg.getBoundingClientRect(); ci = { svgCenter: r.left + r.width / 2 - rowRect.left, svgW: r.width }; }
      });
      results.push({ type: ci ? "Item(folder)" : "Item(leaf)", label: label?.textContent?.trim(), padL: window.getComputedStyle(row).paddingLeft, ...(ci || {}) });
    });
    const lines: any[] = [];
    document.querySelectorAll('[data-slot="tree-section-row"], [data-slot="tree-item-row"]').forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent?.trim();
      const lc = row.querySelector('.absolute.left-0.top-0.bottom-0');
      if (lc) {
        lc.querySelectorAll('.absolute.top-0.bottom-0').forEach(lp => {
          const ld = lp.querySelector('.w-px');
          if (ld) lines.push({ label, relLeft: lp.getBoundingClientRect().left - rowRect.left, style: lp.getAttribute("style") });
        });
      }
    });
    return { rows: results, lines };
  });
  if (measurements.rows.length > 0) {
    console.log("\n=== SKETCHPAD ROWS ===");
    for (const r of measurements.rows) {
      const cc = r.chevCenter ?? r.svgCenter;
      const padL = parseFloat(r.padL);
      const expected = padL + 7;
      if (cc != null) {
        const diff = cc - expected;
        console.log(`${r.type} [${r.label}] padL=${r.padL} chevCenter=${cc.toFixed(2)} expected=${expected.toFixed(2)} diff=${diff.toFixed(2)} ${Math.abs(diff) < 0.5 ? "✓" : "✗"}`);
      } else {
        console.log(`${r.type} [${r.label}] padL=${r.padL}`);
      }
    }
    console.log("\n=== SKETCHPAD LINES ===");
    const seen = new Set();
    for (const l of measurements.lines) {
      const k = l.label + "|" + l.style;
      if (!seen.has(k)) { seen.add(k); console.log(`Row[${l.label}] line relLeft=${l.relLeft.toFixed(2)} style="${l.style}"`); }
    }
  }
  await page.screenshot({ path: "/tmp/sketchpad-tree.png", fullPage: false });
  console.log("\n[DEBUG] Screenshot saved.");
  await browser.close();
}
measure().catch(console.error);
