// 🗺️ Resolves every test-side reference to the ui aggregate to its owner crate and public path, and plans per-crate manifest and use-line edits. JSON to argv[3] (reads rtd-refs.json from argv[2]).
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {loadCrate} from "./rtd-world.ts";
import {ROOT, verdictOf} from "./rtd-verdict.ts";

const refsAll = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const wsAll = (Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
const depsOf = (manifest: string) => {
  const t = Bun.TOML.parse(readFileSync(join(ROOT, manifest), "utf8")) as any;
  const dir = join(ROOT, manifest, "..");
  const have = new Map<string, {key: string; table: string}>();
  const note = (tbl: any, table: string) => {
    for (const [k, raw] of Object.entries(tbl ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const p = spec.path;
      if (typeof p === "string") { const c = loadCrate(join(dir, p)); if (c && !have.has(c.pkg)) have.set(c.pkg, {key: k, table}); }
      else if (spec.workspace === true) {
        const ws = wsAll[spec.package ?? k];
        if (ws?.path) { const c = loadCrate(join(ROOT, ws.path)); if (c && !have.has(c.pkg)) have.set(c.pkg, {key: k, table}); }
      }
    }
  };
  note(t.dependencies, "dependencies"); note(t["dev-dependencies"], "dev-dependencies");
  for (const [tg, b] of Object.entries(t.target ?? {}) as [string, any][]) { note((b as any).dependencies, `target.${tg}.dependencies`); note((b as any)["dev-dependencies"], `target.${tg}.dev-dependencies`); }
  return {have, dir};
};

const plan: any[] = [];
for (const c of refsAll) {
  const testRefs = c.refs.filter((r: any) => r.where === "testfile" || r.where === "testregion");
  const featureRefs = c.refs.filter((r: any) => r.where === "testregion-feature");
  const normalRefs = c.refs.filter((r: any) => r.where === "normal" || r.where === "testregion-feature");
  const out: any = {crate: c.crate, manifest: c.manifest, rootCfgTest: !!c.rootCfgTest, normalRefs: normalRefs.length, featureGatedRefs: featureRefs.length, testRefs: testRefs.length, edits: [], leaves: []};
  const { have, dir } = depsOf(c.manifest);
  out.have = Object.fromEntries([...have].map(([k, v]) => [k, v]));
  const testkitTokens = c.tokens.filter((t: any) => (t.where === "testfile" || t.where === "testregion") && !(t.token === "publish_document" && t.args === 1));
  out.testkitTokens = testkitTokens.length;
  for (const r of testRefs) {
    if (r.form === "path") {
      const segs = r.segs.slice();
      const v = verdictOf(segs);
      out.edits.push({file: r.file, line: r.line, form: "path", where: r.where, verdict: v});
    } else if (r.form === "use") {
      const leaves = (r.tree as any[]).map(l => ({...l, segsRel: l.segs.slice(1), verdict: l.kind === "glob" ? undefined : verdictOf(l.segs.slice(1))}));
      out.edits.push({file: r.file, line: r.line, form: "use", where: r.where, vis: r.vis, stmt: r.stmt, leaves});
    } else out.edits.push({file: r.file, line: r.line, form: r.form, where: r.where, text: r.text});
  }
  void dir;
  plan.push(out);
}
writeFileSync(process.argv[3]!, JSON.stringify(plan, null, 1));
const st = new Map<string, number>();
for (const p of plan) for (const e of p.edits) {
  if (e.form === "path") st.set(e.verdict.status, (st.get(e.verdict.status) ?? 0) + 1);
  else if (e.form === "use") for (const l of e.leaves) st.set("use:" + (l.verdict?.status ?? l.kind), (st.get("use:" + (l.verdict?.status ?? l.kind)) ?? 0) + 1);
  else st.set("form:" + e.form, (st.get("form:" + e.form) ?? 0) + 1);
}
console.log(Object.fromEntries(st));

