// #region 🧲Header
// semio-algorithms Kit/Store: align JS/C# kit JSON with semio/rs `*FullDto` (serde) expectations.
// The metabolism fixture uses id-only refs, `name` where Rust wants `code`/`url`/`path`, etc.
// #endregion

type AnyRec = Record<string, unknown>;

function isObj(x: unknown): x is AnyRec {
  return x != null && typeof x === "object" && !Array.isArray(x);
}

/** `PropFullDto`: `key` + `value`; C# often has `quality: { id }` instead of `key`. */
function fixPropsList(props: unknown, qualityKeyById: Map<string, string>): void {
  if (!Array.isArray(props)) return;
  for (const p of props) {
    if (!isObj(p)) continue;
    const q = p.quality;
    if (isObj(q) && typeof q.id === "string") {
      const kid = qualityKeyById.get(q.id);
      if (kid != null && kid !== "" && (p.key == null || p.key === "")) {
        p.key = kid;
      }
    }
    if (p.key == null) p.key = "";
    if (p.value == null) p.value = "";
  }
}

function expandTagList(tags: unknown, tagNameById: Map<string, string>): void {
  if (!Array.isArray(tags)) return;
  for (const t of tags) {
    if (!isObj(t) || typeof t.id !== "string") continue;
    if (t.name == null || t.name === "") {
      t.name = tagNameById.get(t.id) ?? "";
    }
  }
}

function expandAuthorList(authors: unknown, byId: Map<string, { name: string; email: string }>): void {
  if (!Array.isArray(authors)) return;
  for (const a of authors) {
    if (!isObj(a) || typeof a.id !== "string") continue;
    const full = byId.get(a.id);
    if (full) {
      a.name = full.name;
      a.email = full.email;
    } else {
      if (a.name == null) a.name = "";
      if (a.email == null) a.email = "";
    }
  }
}

function expandConceptList(concepts: unknown, byId: Map<string, { name: string; description: string | undefined }>): void {
  if (!Array.isArray(concepts)) return;
  for (const c of concepts) {
    if (!isObj(c) || typeof c.id !== "string") continue;
    const full = byId.get(c.id);
    if (full) {
      c.name = full.name;
      if (c.description == null && full.description != null) c.description = full.description;
    } else if (c.name == null) {
      c.name = "";
    }
  }
}

function fixConnectors(connectors: unknown, qualityKeyById: Map<string, string>): void {
  if (!Array.isArray(connectors)) return;
  for (const c of connectors) {
    if (!isObj(c)) continue;
    if (c.code == null || c.code === "") {
      const n = c.name;
      c.code = typeof n === "string" && n.length > 0 ? n : String(c.id ?? "");
    }
    fixPropsList(c.props, qualityKeyById);
  }
}

function fixRepresentation(rep: AnyRec, tagNameById: Map<string, string>): void {
  if (rep.url == null || rep.url === "") {
    const n = rep.name;
    const blob = rep.blob;
    rep.url = typeof n === "string" && n.length > 0 ? n : typeof blob === "string" && blob.length > 0 ? blob : "";
  }
  expandTagList(rep.tags, tagNameById);
}

function fixType(t: AnyRec, tagNameById: Map<string, string>, authorById: Map<string, { name: string; email: string }>, conceptById: Map<string, { name: string; description: string | undefined }>, qualityKeyById: Map<string, string>): void {
  fixConnectors(t.connectors, qualityKeyById);
  if (Array.isArray(t.representations)) {
    for (const r of t.representations) {
      if (isObj(r)) fixRepresentation(r, tagNameById);
    }
  }
  expandAuthorList(t.authors, authorById);
  expandConceptList(t.concepts, conceptById);
  expandTagList(t.tags, tagNameById);
}

function fixPiece(piece: AnyRec, qualityKeyById: Map<string, string>): void {
  fixPropsList(piece.props, qualityKeyById);
  // C# uses `isHidden` / `isLocked`; `PieceFullDto` has `hidden` / `locked`.
  if (piece.isHidden != null && piece.hidden == null) piece.hidden = piece.isHidden;
  if (piece.isLocked != null && piece.locked == null) piece.locked = piece.isLocked;
}

/** `LayerFullDto` requires `name`; exports often use `path` (IFC / tree path) only. */
function fixLayers(layers: unknown): void {
  if (!Array.isArray(layers)) return;
  for (const layer of layers) {
    if (!isObj(layer)) continue;
    if (layer.name == null || layer.name === "") {
      const path = layer.path;
      layer.name = typeof path === "string" && path.length > 0 ? path : String(layer.id ?? "");
    }
  }
}

function fixDesign(d: AnyRec, tagNameById: Map<string, string>, authorById: Map<string, { name: string; email: string }>, conceptById: Map<string, { name: string; description: string | undefined }>, qualityKeyById: Map<string, string>): void {
  expandAuthorList(d.authors, authorById);
  expandConceptList(d.concepts, conceptById);
  expandTagList(d.tags, tagNameById);
  fixLayers(d.layers);
  if (Array.isArray(d.pieces)) {
    for (const p of d.pieces) {
      if (isObj(p)) fixPiece(p, qualityKeyById);
    }
  }
}

function fixFiles(files: unknown): void {
  if (!Array.isArray(files)) return;
  for (const f of files) {
    if (!isObj(f)) continue;
    if (f.url == null || f.url === "") {
      const blob = f.blob;
      const n = f.name;
      f.url = typeof blob === "string" && blob.length > 0 ? blob : typeof n === "string" && n.length > 0 ? n : "";
    }
  }
}

function fixFolders(folders: unknown): void {
  if (!Array.isArray(folders)) return;
  for (const f of folders) {
    if (!isObj(f)) continue;
    if (f.path == null || f.path === "") {
      const n = f.name;
      f.path = typeof n === "string" && n.length > 0 ? n : "";
    }
  }
}

function ensureQualityKeys(qualities: unknown): void {
  if (!Array.isArray(qualities)) return;
  for (const q of qualities) {
    if (!isObj(q) || typeof q.id !== "string") continue;
    if (q.key == null || q.key === "") {
      const n = q.name;
      q.key = typeof n === "string" && n.length > 0 ? n : q.id;
    }
  }
}

/**
 * In-memory (Storybook / WASM) — maps C#/JS export shape to `KitFullDto` as consumed by `semio/rs`.
 */
export function normalizeKitJsonForSemioRsWasm(kit: unknown): unknown {
  if (!isObj(kit)) {
    return kit;
  }
  const clone = JSON.parse(JSON.stringify(kit)) as AnyRec;

  const tagNameById = new Map<string, string>();
  if (Array.isArray(clone.tags)) {
    for (const t of clone.tags) {
      if (isObj(t) && typeof t.id === "string" && typeof t.name === "string") {
        tagNameById.set(t.id, t.name);
      }
    }
  }

  const authorById = new Map<string, { name: string; email: string }>();
  if (Array.isArray(clone.authors)) {
    for (const a of clone.authors) {
      if (isObj(a) && typeof a.id === "string") {
        authorById.set(a.id, {
          name: typeof a.name === "string" ? a.name : "",
          email: typeof a.email === "string" ? a.email : "",
        });
      }
    }
  }

  const conceptById = new Map<string, { name: string; description: string | undefined }>();
  if (Array.isArray(clone.concepts)) {
    for (const c of clone.concepts) {
      if (isObj(c) && typeof c.id === "string") {
        conceptById.set(c.id, {
          name: typeof c.name === "string" ? c.name : "",
          description: typeof c.description === "string" ? c.description : undefined,
        });
      }
    }
  }

  const qualityKeyById = new Map<string, string>();
  if (Array.isArray(clone.qualities)) {
    for (const q of clone.qualities) {
      if (isObj(q) && typeof q.id === "string") {
        const k = q.key;
        const n = q.name;
        const key = typeof k === "string" && k.length > 0 ? k : typeof n === "string" && n.length > 0 ? n : String(q.id);
        qualityKeyById.set(q.id, key);
      }
    }
  }
  ensureQualityKeys(clone.qualities);

  fixFiles(clone.files);
  fixFolders(clone.folders);
  fixPropsList(clone.props, qualityKeyById);

  if (Array.isArray(clone.types)) {
    for (const t of clone.types) {
      if (isObj(t)) fixType(t, tagNameById, authorById, conceptById, qualityKeyById);
    }
  }
  if (Array.isArray(clone.designs)) {
    for (const d of clone.designs) {
      if (isObj(d)) fixDesign(d, tagNameById, authorById, conceptById, qualityKeyById);
    }
  }

  return clone;
}
