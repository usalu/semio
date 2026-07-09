const NS = "http://www.w3.org/2000/svg";

export type Placement = {
  href: string;
  x: number;
  y: number;
  title?: string;
};

export type NakaginScene = {
  width: number;
  height: number;
  defsMarkup: string;
  placements: Placement[];
  source: string;
};

export async function loadNakaginScene(url = "/nakagin-capsule-tower.svg"): Promise<NakaginScene> {
  const source = await fetch(url).then((r) => r.text());
  const doc = new DOMParser().parseFromString(source, "image/svg+xml");
  const svg = doc.documentElement;

  const width = Number(svg.getAttribute("width") ?? 0);
  const height = Number(svg.getAttribute("height") ?? 0);

  const defs = doc.querySelector("defs");
  if (!defs) throw new Error("No <defs> found in source SVG");

  const defsMarkup = Array.from(defs.children)
    .filter((el) => {
      const id = el.getAttribute("id");
      return !!id && id !== "connections" && id !== "pieces";
    })
    .map((el) => el.outerHTML)
    .join("\n");

  const placements: Placement[] = Array.from(doc.querySelectorAll("#pieces > use")).map((el) => ({
    href: el.getAttribute("href") || el.getAttribute("xlink:href") || "",
    x: Number(el.getAttribute("x") || 0),
    y: Number(el.getAttribute("y") || 0),
    title: el.querySelector("title")?.textContent ?? undefined,
  }));

  return { width, height, defsMarkup, placements, source };
}

export function buildSceneSvg(scene: NakaginScene): string {
  const uses = scene.placements
    .map((p) => {
      const title = p.title ? `<title>${escapeXml(p.title)}</title>` : "";
      return `<use href="${p.href}" x="${p.x}" y="${p.y}">${title}</use>`;
    })
    .join("\n");

  return `
    <svg xmlns="${NS}" width="${scene.width}" height="${scene.height}"
         viewBox="0 0 ${scene.width} ${scene.height}">
      <defs>
        ${scene.defsMarkup}
      </defs>
      ${uses}
    </svg>
  `.trim();
}

function escapeXml(s: string): string {
  return s.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;").replaceAll("'", "&apos;");
}
