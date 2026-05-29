import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { BreachRecord } from "./breach.ts";
import { getWorkspaceRoot, runCliGraphql } from "./cli.ts";

const NODE_QUERY = `
query NodeQ($id: ID!) {
  node(id: $id) {
    __typename
    ... on File {
      id path name extension kind
      sections { id name path range { start end } }
      definitions { id name kind range { start end } }
    }
    ... on Folder {
      id path name
    }
    ... on Bundle {
      id name root kind technologyName
    }
    ... on Technology {
      id name kind root
    }
    ... on Section {
      id name path
      range { start end }
      file { path }
      definitions { id name kind range { start end } }
    }
    ... on Definition {
      id name kind
      range { start end }
      file { path }
    }
  }
}
`;

export type GraphNode = Record<string, unknown> & { __typename?: string };

/** 🧷BaseLinter holds shared repo-root + graphql helpers for lint scripts. */
export abstract class BaseLinter {
  constructor(
    readonly entityId: string,
    protected readonly repoRoot: string = getWorkspaceRoot(),
  ) {}

  protected gql<T = unknown>(query: string, variables: Record<string, unknown> = {}): T {
    return runCliGraphql(query, variables, { repoRoot: this.repoRoot }) as T;
  }

  protected loadNode(): GraphNode {
    const data = this.gql<{ node: GraphNode | null }>(NODE_QUERY, { id: this.entityId });
    const n = data.node;
    if (!n || !n.__typename) {
      throw new Error(`[linter] node not found for id ${this.entityId}`);
    }
    return n;
  }

  /** 🚫Builds a breach with default scope = entity id. */
  breach(p: Omit<BreachRecord, "scope"> & { scope?: string }): BreachRecord {
    const { scope, ...rest } = p;
    return {
      ...rest,
      scope: scope ?? this.entityId,
    };
  }

}

/** 🏗️TechnologyLinter queries a technology node by id. */
export class TechnologyLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Technology") {
      throw new Error(`[TechnologyLinter] expected Technology, got ${this.node.__typename}`);
    }
    return this.node;
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
  }

  root(): string {
    return String(this.load().root ?? "");
  }

  /** 📦Lists bundle rows for this technology. */
  bundles(): GraphNode[] {
    const data = this.gql<{ bundles: GraphNode[] }>(
      `query B { bundles { id name root kind technologyName } }`,
    );
    const tech = this.name();
    return (data.bundles ?? []).filter((b) => String(b.technologyName ?? "") === tech);
  }
}

/** 📦BundleLinter queries a bundle node by id. */
export class BundleLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Bundle") {
      throw new Error(`[BundleLinter] expected Bundle, got ${this.node.__typename}`);
    }
    return this.node;
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  root(): string {
    return String(this.load().root ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
  }

  technologyName(): string {
    return String(this.load().technologyName ?? "");
  }
}

/** 📁FolderLinter queries a folder node by id. */
export class FolderLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Folder") {
      throw new Error(`[FolderLinter] expected Folder, got ${this.node.__typename}`);
    }
    return this.node;
  }

  path(): string {
    return String(this.load().path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }
}

/** 📄FileLinter queries a file node by id and reads bytes from disk. */
export class FileLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "File") {
      throw new Error(`[FileLinter] expected File, got ${this.node.__typename}`);
    }
    return this.node;
  }

  path(): string {
    return String(this.load().path ?? "").replaceAll("\\", "/");
  }

  ext(): string {
    return String(this.load().extension ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
  }

  content(): string {
    const p = this.path();
    return readFileSync(join(this.repoRoot, p), "utf8");
  }

  lines(): string[] {
    return this.content().split(/\r?\n/);
  }

  sections(): GraphNode[] {
    return (this.load().sections as GraphNode[]) ?? [];
  }

  definitions(): GraphNode[] {
    return (this.load().definitions as GraphNode[]) ?? [];
  }
}

/** 🔖SectionLinter queries a section node by id. */
export class SectionLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Section") {
      throw new Error(`[SectionLinter] expected Section, got ${this.node.__typename}`);
    }
    return this.node;
  }

  filePath(): string {
    const f = this.load().file as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  sectionPath(): string {
    return String(this.load().path ?? "");
  }

  startLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.start ?? 0);
  }

  endLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.end ?? 0);
  }

  content(): string {
    const full = readFileSync(join(this.repoRoot, this.filePath()), "utf8");
    const lines = full.split(/\r?\n/);
    const s = this.startLine();
    const e = this.endLine();
    if (s <= 0 || e < s) return "";
    return lines.slice(s - 1, e).join("\n");
  }

  definitions(): GraphNode[] {
    return (this.load().definitions as GraphNode[]) ?? [];
  }
}

/** 🏷️DefinitionLinter queries a definition node by id. */
export class DefinitionLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Definition") {
      throw new Error(`[DefinitionLinter] expected Definition, got ${this.node.__typename}`);
    }
    return this.node;
  }

  filePath(): string {
    const f = this.load().file as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
  }

  startLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.start ?? 0);
  }

  endLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.end ?? 0);
  }

  content(): string {
    const full = readFileSync(join(this.repoRoot, this.filePath()), "utf8");
    const lines = full.split(/\r?\n/);
    const s = this.startLine();
    const e = this.endLine();
    if (s <= 0 || e < s) return "";
    return lines.slice(s - 1, e).join("\n");
  }
}

/** 🔎Resolves folder path to folder graphql row (for script.ts policy placement). */
export function resolveFolderByPath(repoRoot: string, folderPath: string): GraphNode {
  const rel = folderPath.replaceAll("\\", "/").replace(/^\/+/, "");
  const data = runCliGraphql(
    `query F($p: String!) { folder(path: $p) { __typename id path name } }`,
    { p: rel },
    { repoRoot },
  ) as { folder: GraphNode };
  if (!data.folder?.id) throw new Error(`[linter] folder not found for path ${rel}`);
  return data.folder;
}

/** 🔎Resolves bundle name like `repo/client` to bundle id. */
export function resolveBundleByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(
    `query B($n: String!) { bundle(name: $n) { __typename id name root kind technologyName } }`,
    { n: name },
    { repoRoot },
  ) as { bundle: GraphNode };
  if (!data.bundle?.id) throw new Error(`[linter] bundle not found for name ${name}`);
  return data.bundle;
}

/** 🔎Resolves technology folder name (e.g. `repo`) to technology id. */
export function resolveTechnologyByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(
    `query T { technologies { id name root kind } }`,
    {},
    { repoRoot },
  ) as { technologies: GraphNode[] };
  const hit = (data.technologies ?? []).find((t) => String(t.name ?? "") === name);
  if (!hit?.id) throw new Error(`[linter] technology not found for name ${name}`);
  return hit;
}
