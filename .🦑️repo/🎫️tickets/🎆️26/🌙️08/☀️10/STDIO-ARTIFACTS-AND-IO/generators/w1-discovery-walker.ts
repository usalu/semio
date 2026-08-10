/** 🌳️ Level descriptor: fixed allowlist or wildcard (`*` = any emoji-prefixed slug dir). */
type ArtifactFacetLevel =
  | { readonly kind: "fixed"; readonly dirs: readonly string[] }
  | { readonly kind: "wildcard" }
  | { readonly kind: "none" };

/** 🂡 Whether a dir name is an emoji-prefixed slug (requires U+FE0F in the emoji prefix). */
function isEmojiPrefixedSlugDir(name: string): boolean {
  return /\p{Extended_Pictographic}\uFE0F/u.test(name);
}

/** 🌳️ Declared child level of a path under an artifact root (parents are `/`-segments already accepted). */
function artifactFacetChildLevel(parents: readonly string[], taxonomy: Taxonomy): ArtifactFacetLevel {
  if (parents.length === 0) return { kind: "fixed", dirs: taxonomy.artifactComponentDirs };
  const root = parents[0]!;
  const a = parents[1];
  const b = parents[2];
  const c = parents[3];
  if (parents.length === 1) {
    if (root === "🧬️schema") return { kind: "fixed", dirs: taxonomy.schemaChildDirs ?? [] };
    if (root === "🚪️io") return { kind: "fixed", dirs: taxonomy.ioDirectionDirs ?? [] };
    return { kind: "none" };
  }
  if (root === "🧬️schema") {
    if (parents.length === 2 && (taxonomy.schemaChildDirs ?? []).includes(a!)) {
      if (a === "🧬️mutations") return { kind: "fixed", dirs: [...(taxonomy.representationDirs ?? []), "*"] };
      return { kind: "fixed", dirs: taxonomy.representationDirs ?? [] };
    }
    if (parents.length === 3 && a === "🧬️mutations") {
      if ((taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
      return { kind: "fixed", dirs: taxonomy.mutationChildDirs ?? [] };
    }
    if (parents.length === 3 && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
    if (parents.length === 4 && a === "🧬️mutations") return { kind: "none" };
    return { kind: "none" };
  }
  if (root === "🚪️io") {
    const directions = taxonomy.ioDirectionDirs ?? [];
    const childMap = taxonomy.ioDirectionChildDirs ?? {};
    if (parents.length === 2 && directions.includes(a!)) {
      const child = childMap[a!];
      return child ? { kind: "fixed", dirs: [child] } : { kind: "none" };
    }
    if (parents.length === 3 && directions.includes(a!) && childMap[a!] === b) {
      return { kind: "fixed", dirs: [taxonomy.artifactsDirName] };
    }
    if (parents.length === 4 && b === childMap[a!] && c === taxonomy.artifactsDirName) return { kind: "wildcard" };
    if (parents.length === 5) return { kind: "none" };
    return { kind: "none" };
  }
  return { kind: "none" };
}

/** 🌳️ Declared children of a nesting artifact facet path (`/`-joined parents), empty when leaves-only. */
function artifactFacetChildDirs(facetPath: string, taxonomy: Taxonomy): readonly string[] {
  const parents = facetPath ? facetPath.split("/") : [];
  const level = artifactFacetChildLevel(parents, taxonomy);
  if (level.kind !== "fixed") return [];
  return level.dirs.filter((d) => d !== "*");
}

/** 🌳️ Declared children of a nesting app facet. */
function appFacetChildDirs(facet: string, taxonomy: Taxonomy): readonly string[] {
  if (facet === "🎚️config") return taxonomy.configChildDirs ?? [];
  if (facet === "👥️presence") return taxonomy.presenceChildDirs ?? [];
  return [];
}

/** 🧭️ Whether a `/`-joined facet path such as `🎚️config/🧬️schema` walks only declared dirs from an app (or shared config owner). */
export function appFacetPathIsDeclared(facetPath: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const [root, ...rest] = facetPath.split("/");
  if (!root || !taxonomy.appComponentDirs.includes(root)) return false;
  let parent = root;
  for (const segment of rest) {
    if (!appFacetChildDirs(parent, taxonomy).includes(segment)) return false;
    parent = segment;
  }
  return true;
}

/** 🧭️ Whether a `/`-joined facet path walks only declared dirs from an artifact root (supports `*` wildcard levels). */
export function artifactFacetPathIsDeclared(facetPath: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const [root, ...rest] = facetPath.split("/");
  if (!root || !taxonomy.artifactComponentDirs.includes(root)) return false;
  const parents: string[] = [root];
  for (const segment of rest) {
    const level = artifactFacetChildLevel(parents, taxonomy);
    if (level.kind === "none") return false;
    if (level.kind === "wildcard") {
      if (!isEmojiPrefixedSlugDir(segment)) return false;
    } else {
      const dirs = level.dirs;
      const allowWildcard = dirs.includes("*");
      const fixed = dirs.filter((d) => d !== "*");
      if (!(fixed.includes(segment) || (allowWildcard && isEmojiPrefixedSlugDir(segment)))) return false;
    }
    parents.push(segment);
  }
  return true;
}

