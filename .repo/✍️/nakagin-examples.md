# Recreating `nakagin-capsule-tower.svg` in JS/TS

The source SVG is already a scene graph:

- canvas size: `367.416 × 2169.141`
- reusable defs: `icon`, `root`, plus 12 piece symbols
- placements: 181 `<use>` instances

So the cleanest way to "recreate it in code" is:

1. parse the source SVG once
2. extract its reusable defs and placement list
3. rebuild the scene with the target library

That gives you a faithful result without hand-transcribing hundreds of paths.

---

## Shared helper

```ts
// nakagin-shared.ts
const NS = 'http://www.w3.org/2000/svg';

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

export async function loadNakaginScene(
  url = '/nakagin-capsule-tower.svg',
): Promise<NakaginScene> {
  const source = await fetch(url).then((r) => r.text());
  const doc = new DOMParser().parseFromString(source, 'image/svg+xml');
  const svg = doc.documentElement;

  const width = Number(svg.getAttribute('width') ?? 0);
  const height = Number(svg.getAttribute('height') ?? 0);

  const defs = doc.querySelector('defs');
  if (!defs) throw new Error('No <defs> found in source SVG');

  // Keep everything needed to resolve <use href="#..."> except the placement groups.
  const defsMarkup = Array.from(defs.children)
    .filter((el) => {
      const id = el.getAttribute('id');
      return !!id && id !== 'connections' && id !== 'pieces';
    })
    .map((el) => el.outerHTML)
    .join('\n');

  const placements: Placement[] = Array.from(
    doc.querySelectorAll('#pieces > use'),
  ).map((el) => ({
    href: el.getAttribute('href') || el.getAttribute('xlink:href') || '',
    x: Number(el.getAttribute('x') || 0),
    y: Number(el.getAttribute('y') || 0),
    title: el.querySelector('title')?.textContent ?? undefined,
  }));

  return { width, height, defsMarkup, placements, source };
}

export function buildSceneSvg(scene: NakaginScene): string {
  const uses = scene.placements
    .map((p) => {
      const title = p.title ? `<title>${escapeXml(p.title)}</title>` : '';
      return `<use href="${p.href}" x="${p.x}" y="${p.y}">${title}</use>`;
    })
    .join('\n');

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
  return s
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&apos;');
}
```

---

## 1) SVG.js — exact recreation

```ts
import { SVG } from '@svgdotjs/svg.js';
import { loadNakaginScene } from './nakagin-shared';

const NS = 'http://www.w3.org/2000/svg';

export async function renderWithSvgJs(container: HTMLElement) {
  const scene = await loadNakaginScene('/nakagin-capsule-tower.svg');

  const draw = SVG()
    .addTo(container)
    .size(scene.width, scene.height)
    .viewbox(0, 0, scene.width, scene.height);

  // Import defs from the original file.
  draw.defs().node.innerHTML = scene.defsMarkup;

  // Recreate placements with <use>.
  for (const p of scene.placements) {
    const use = document.createElementNS(NS, 'use');
    use.setAttribute('href', p.href);
    use.setAttribute('x', String(p.x));
    use.setAttribute('y', String(p.y));
    if (p.title) {
      const title = document.createElementNS(NS, 'title');
      title.textContent = p.title;
      use.appendChild(title);
    }
    draw.node.appendChild(use);
  }
}
```

Why this version works well: the original art is already symbol-based, and SVG.js is perfectly happy to host imported defs plus native `<use>` placements.

---

## 2) D3 — exact recreation from data join

```ts
import * as d3 from 'd3';
import { loadNakaginScene } from './nakagin-shared';

const NS = 'http://www.w3.org/2000/svg';

export async function renderWithD3(container: HTMLElement) {
  const scene = await loadNakaginScene('/nakagin-capsule-tower.svg');

  const svg = d3
    .select(container)
    .append('svg')
    .attr('xmlns', NS)
    .attr('width', scene.width)
    .attr('height', scene.height)
    .attr('viewBox', `0 0 ${scene.width} ${scene.height}`);

  svg.append('defs').html(scene.defsMarkup);

  svg.selectAll<SVGUseElement, typeof scene.placements[number]>('use.piece')
    .data(scene.placements)
    .join('use')
    .attr('class', 'piece')
    .attr('href', (d) => d.href)
    .attr('x', (d) => d.x)
    .attr('y', (d) => d.y)
    .each(function (d) {
      if (!d.title) return;
      const title = document.createElementNS(NS, 'title');
      title.textContent = d.title;
      this.appendChild(title);
    });
}
```

D3 is especially nice here because the scene is naturally "data + repeated symbol placement".

---

## 3) Two.js — easiest exact route is to let Two load/interpret the SVG

```ts
import Two from 'two.js';

export async function renderWithTwoJs(container: HTMLElement) {
  const source = await fetch('/nakagin-capsule-tower.svg').then((r) => r.text());

  const two = new Two({
    type: Two.Types.svg,
    width: 367.416,
    height: 2169.141,
  }).appendTo(container);

  two.load(source, (group) => {
    // Optional: position / style the imported group
    group.translation.set(0, 0);
    two.update();
  });
}
```

If you want to preprocess the scene first, you can also parse the SVG into a DOM node and pass it to `two.interpret(...)`.

---

## 4) Rough.js — best as a stylized approximation

Rough.js is not the best match for this asset because the original depends on exact SVG symbol reuse and embedded mini-illustrations inside each circular node. The most natural Rough.js version is to redraw the tower as sketchy capsules.

```ts
import rough from 'roughjs';
import { loadNakaginScene } from './nakagin-shared';

const NS = 'http://www.w3.org/2000/svg';

function append(node: SVGGElement, child: SVGGElement | SVGElement) {
  node.appendChild(child as unknown as Node);
}

function drawRoughCapsule(
  rc: ReturnType<typeof rough.svg>,
  x: number,
  y: number,
  variant: string,
) {
  const g = document.createElementNS(NS, 'g');
  const cx = x + 24;
  const cy = y + 24;

  append(
    g,
    rc.circle(cx, cy, 47, {
      fill: 'white',
      fillStyle: 'solid',
      stroke: 'black',
      roughness: 0.6,
      bowing: 0.4,
    }),
  );

  // Front panel
  append(
    g,
    rc.rectangle(cx - 10, cy - 6, 16, 14, {
      stroke: 'black',
      roughness: 0.5,
    }),
  );

  // Side porthole
  append(
    g,
    rc.circle(cx + 10, cy + 7, 8, {
      stroke: 'black',
      roughness: 0.5,
    }),
  );

  // Roof / seam lines; flip depending on variant family
  const rightFacing = /\/|J|p|s/.test(variant);

  append(
    g,
    rc.line(
      cx - 15,
      cy - 14,
      rightFacing ? cx + 2 : cx - 2,
      cy - 4,
      { roughness: 0.6 },
    ),
  );

  append(
    g,
    rc.line(
      cx - 2,
      cy - 4,
      rightFacing ? cx + 14 : cx - 14,
      cy + 8,
      { roughness: 0.6 },
    ),
  );

  return g;
}

function drawRoughTambour(
  rc: ReturnType<typeof rough.svg>,
  x: number,
  y: number,
) {
  const g = document.createElementNS(NS, 'g');
  const cx = x + 24;
  const cy = y + 24;

  append(
    g,
    rc.circle(cx, cy, 47, {
      fill: 'white',
      fillStyle: 'solid',
      stroke: 'black',
      roughness: 0.5,
    }),
  );

  for (let row = -2; row <= 2; row++) {
    for (let col = -2; col <= 2; col++) {
      append(
        g,
        rc.rectangle(cx - 12 + col * 5, cy - 12 + row * 5, 5, 5, {
          stroke: 'black',
          roughness: 0.3,
        }),
      );
    }
  }

  return g;
}

export async function renderWithRoughJs(container: HTMLElement) {
  const scene = await loadNakaginScene('/nakagin-capsule-tower.svg');

  const svg = document.createElementNS(NS, 'svg');
  svg.setAttribute('width', String(scene.width));
  svg.setAttribute('height', String(scene.height));
  svg.setAttribute('viewBox', `0 0 ${scene.width} ${scene.height}`);
  container.appendChild(svg);

  const rc = rough.svg(svg);

  for (const p of scene.placements) {
    let node: SVGGElement;

    if (p.href.includes('Tambour')) {
      node = drawRoughTambour(rc, p.x, p.y);
    } else if (
      p.href.includes('Capsule') ||
      p.href === '#root' ||
      p.href.includes('Base') ||
      p.href.includes('Capital')
    ) {
      node = drawRoughCapsule(rc, p.x, p.y, p.href);
    } else {
      node = drawRoughCapsule(rc, p.x, p.y, p.href);
    }

    svg.appendChild(node);
  }
}
```

That gives you the same layout, but translated into Rough.js’s hand-drawn language.

---

## 5) Paper.js — import the generated scene into the Paper project

Paper.js is canvas-first, so the most ergonomic approach is to rebuild the SVG string and import it into Paper’s scene graph.

```ts
import paper from 'paper';
import { buildSceneSvg, loadNakaginScene } from './nakagin-shared';

export async function renderWithPaperJs(canvas: HTMLCanvasElement) {
  const scene = await loadNakaginScene('/nakagin-capsule-tower.svg');
  const svgMarkup = buildSceneSvg(scene);

  paper.setup(canvas);
  paper.view.viewSize = new paper.Size(scene.width, scene.height);

  paper.project.importSVG(svgMarkup, {
    insert: true,
    expandShapes: false,
  });

  paper.view.update();
}
```

If you want a more "Paper-native" version, you can import each symbol once, wrap it in `new paper.SymbolDefinition(...)`, and then place `SymbolItem`s from `scene.placements`.

---

## 6) Snap.svg — exact recreation with defs + uses

```ts
import Snap from 'snapsvg';
import { loadNakaginScene } from './nakagin-shared';

const NS = 'http://www.w3.org/2000/svg';

export async function renderWithSnap(container: HTMLElement) {
  const scene = await loadNakaginScene('/nakagin-capsule-tower.svg');

  const s = Snap(scene.width, scene.height);
  s.attr({
    viewBox: `0 0 ${scene.width} ${scene.height}`,
    width: scene.width,
    height: scene.height,
  });

  const defs = document.createElementNS(NS, 'defs');
  defs.innerHTML = scene.defsMarkup;
  s.node.appendChild(defs);

  for (const p of scene.placements) {
    const use = document.createElementNS(NS, 'use');
    use.setAttribute('href', p.href);
    use.setAttribute('x', String(p.x));
    use.setAttribute('y', String(p.y));
    if (p.title) {
      const title = document.createElementNS(NS, 'title');
      title.textContent = p.title;
      use.appendChild(title);
    }
    s.node.appendChild(use);
  }

  container.appendChild(s.node);
}
```

---

## Which one I would use

- **Exact 1:1 recreation:** SVG.js, D3, or Snap.svg
- **Least code for exact visual import:** Two.js or Paper.js
- **Stylized reinterpretation:** Rough.js

---

## Practical recommendation

If your goal is **faithful output** and easy later editing, use this structure:

- keep the original SVG as the source of truth
- parse it into `{ defsMarkup, placements }`
- render placements from data
- only switch libraries for ergonomics / animation / interaction

That keeps the artwork maintainable and avoids manually rewriting dozens of repeated symbols.
