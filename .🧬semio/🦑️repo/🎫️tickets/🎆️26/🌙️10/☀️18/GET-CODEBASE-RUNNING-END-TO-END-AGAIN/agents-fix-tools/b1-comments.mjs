import { load, set, edit } from "./lib.mjs";
import { scan } from "./scan-comments.mjs";

const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
export const COMMENT_FILES = {
  verify: L + "🎛️commands/✅️verify/🟦️.ts",
  dev: L + "🎛️commands/🛠️dev/🟦️.ts",
  schema: L + "🎛️commands/🧬️schema/🟦️.ts",
  nodes: L + "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",
  approvals: L + "🔍️discovery/✅️owned-file-approvals/🟦️.ts",
  evidence: L + "🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts",
  joins: L + "🔍️discovery/🔀️rust-non-repo-join-bases/🟦️.ts",
  markers: L + "🔍️discovery/🪧️owner-markers/🟦️.ts",
  graph: L + "🧹️normalization/🦀️rust-reference-graph/🟦️.ts",
  reach: L + "🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts",
};

export function stripComments(path) {
  const text = load(path);
  const hits = scan(text, path).filter((h) => !h.directive && !h.isDoc && !h.body.startsWith("/**"));
  let out = text;
  for (const h of hits.sort((a, b) => b.pos - a.pos)) {
    const ls = out.lastIndexOf("\n", h.pos - 1) + 1;
    const le = out.indexOf("\n", h.end);
    const before = out.slice(ls, h.pos), after = out.slice(h.end, le < 0 ? out.length : le);
    if (/^\s*$/.test(before) && /^\s*$/.test(after)) out = out.slice(0, ls) + out.slice(le < 0 ? out.length : le + 1);
    else out = out.slice(0, h.pos).replace(/[ \t]+$/, " ") + out.slice(h.end).replace(/^[ \t]+/, "");
  }
  set(path, out);
  return hits.length;
}

export function build() {
  const F = COMMENT_FILES;
  const counts = {};
  for (const [k, p] of Object.entries(F)) counts[k] = stripComments(p);

  edit(F.verify, `  async run(segments: string[]): Promise<void> {
    if (await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", ...segments])) return;`, `  /**
   * 🧭️ Routes one \`verify\` lane to its checks.
   *
   * The \`media-export-encoding\` lane runs \`tsc --strict\` over the codec twin alone: the twin is self-contained by construction, whereas the law imports \`🎠️kernel/🎬️media-export/🟦️.ts\` and would drag the whole renderer graph (and its pre-existing, unrelated Bun-ism diagnostics) into the verdict of the lane.
   *
   * The \`file-open-import\` lane runs \`tsc --strict\` over the PAGE half alone: the page imports nothing, is the one module allowed to name \`document\` on this target, and must type-check against the DOM lib without dragging the renderer graph (and its unrelated diagnostics) into the verdict of the lane.
   */
  async run(segments: string[]): Promise<void> {
    if (await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", ...segments])) return;`);

  edit(F.dev, `  private async parseStorybookSegments(`, `  /** 🔖️ Splits storybook arguments into the scope list and the pass-through arguments. A single scope argument may itself be a comma-list of top-level scopes (\`"ui,framework/os"\`); several space-separated arguments keep the slash-join of hierarchical scopes (\`"framework" "os"\` gives \`"framework/os"\`). */
  private async parseStorybookSegments(`);

  edit(F.nodes, `export async function emojiProjectJsonNodes(`, `/** 🛡️ Infers the project nodes of every authored emoji-named project config. Nx's native walker sometimes hands back lossy paths with U+FFFD where a multi-byte emoji used to be: those are unopenable duplicates of a real file and are skipped. */
export async function emojiProjectJsonNodes(`);

  edit(F.schema, `} catch {
      }`, `} catch {}`);

  edit(F.approvals, `catch { }`, `catch {}`);
  edit(F.approvals, `keyed by the path Git reports relative to \`repoRoot\`; empty outside a work tree. */`, `keyed by the path Git reports relative to \`repoRoot\`; empty outside a work tree, where the platform default is the only authority. */`);

  edit(F.markers, `catch {
  }
  return ownerRel`, `catch {}
  return ownerRel`);
  edit(F.markers, `an emoji-stripped dash-joined owner path, when \`readSemioMarker\` found no explicit \`id\`. */`, `an emoji-stripped dash-joined owner path, when \`readSemioMarker\` found no explicit \`id\`; a manifest that cannot be read or parsed falls through to the path-derived id. */`);

  edit(F.reach, `catch { }
  }
  const breaches`, `catch {}
  }
  const breaches`);
  edit(F.reach, ` * the aggregate's \`x-semio-mutationKinds\` string list, which no consumer reads.
 */`, ` * the aggregate's \`x-semio-mutationKinds\` string list, which no consumer reads. A leaf payload that cannot be read as JSON is not an error here: it is already a schema-parity breach on the leaf.
 */`);

  edit(F.evidence, ` * single real \`impl\` block existing.
 */`, ` * single real \`impl\` block existing.
 *
 * A Rust char literal (\`'"'\`, \`'{'\`, \`'\\n'\`, \`'\\''\`, \`'\\u{1F600}'\`, …) is told from a lifetime/generic tick (\`'a\`, \`'static\`) because a char literal always closes with another \`'\` within a few characters (accounting for escapes) and a lifetime tick never does. Without this, a char literal whose content is itself \`'"'\` gets misread as the START of a double-quoted string, silently swallowing everything up to the next literal \`"\` in the file, which can hide real code (e.g. a later \`impl Trait for ...\`) from every regex-based policy check that runs against the stripped output. A char literal is copied through whole; for a lifetime tick only the tick is copied, because the identifier that follows contains no quote characters.
 */`);

  edit(F.joins, ` * tail expression is proven non-repo count for its callers too, without unbounded recursion. */`, ` * tail expression is proven non-repo count for its callers too, without unbounded recursion.
 *
 * There is no whole-file unpaired-bracket bail (unlike the CARGO_MANIFEST_DIR provers): this function only ever SUPPRESSES a block and never emits a rewrite, and \`inspectRustJoinArgumentSpans\`, which shares this exact tokenizer/pairer and already runs unguarded on every file this feeds, proves the risk is contained; every match still requires the exact adjacent-token shape to fire. */`);
  edit(F.joins, `   * past it (before any \`.join()\` chain), or null if unrecognized — callers must then still block. */`, `   * past it (before any \`.join()\` chain), or null if unrecognized — callers must then still block.
   *
   * \`<any path>::test_support::tempdir()\` is such a root: a project-owned wrapper, uniquely defined once (\`🏪️store/🦀️.rs\` \`pub fn tempdir()\`) as \`std::env::temp_dir().join(...)\` and called via varying aliases/qualifiers (\`store::test_support::tempdir()\`, \`crate::os_store::test_support::tempdir()\`, …). It is matched by SUFFIX so any qualifying prefix path is accepted, but a BARE \`tempdir()\` with no \`test_support::\` qualifier ahead of it is never trusted. */`);
  edit(F.joins, `   * since the outer \`visit\` below independently walks this exact same body for its own literal joins. */`, `   * since the outer \`visit\` below independently walks this exact same body for its own literal joins. Each \`let\` proves its right-hand side BEFORE deleting the old binding: \`let x = f(x)\` (re-binding shadowing, e.g. \`PathBuf::from(out_dir)\`) must still see the prior \`x\`. */`);
  edit(F.joins, `  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {`, `  /** 🧭️ Walks one token range, threading \`let\` bindings and descending into nested \`fn\` bodies. A tuple-pattern destructure \`let (a, _, c) = qualifier::fn(..);\` binds each named (non-\`_\`) position whose slot is independently proven non-repo by \`tupleCallNonRepoPositions\`, and any unrecognized shape just binds nothing (the prior, always-safe default). A plain \`let\` proves its right-hand side against the OLD binding of the name (self-shadowing, e.g. \`let out_dir = PathBuf::from(out_dir)\`) before clearing it, because deleting first would make the reference to the shadowed value unprovable. */
  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {`);

  edit(F.graph, `function rustCodeOnlyTextForMacroTrust(source: string): string {`, `/** 🧹️ The code-only token text the macro-trust checks scan: the module-structure-transparent attributes and macros are dropped and every other token is kept.
 *
 * A bare \`!\` immediately after a real identifier is a macro-invocation bang (\`name!\`/\`path::name!\`), the only Rust construct a lone \`!\` token can start that is capable of expanding to an item, and stays in the text for the macro-trust checks. Every other bare \`!\` is prefix boolean negation (\`!expr\`), which the grammar restricts to expression position: it can never expand into, or hide, a \`mod\`. A RESERVED KEYWORD is never a valid macro name, so \`if !(…)\`/\`while !(…)\`/\`return !x\` are negation too, even though the tokenizer gives keywords \`kind: "identifier"\` the same as real identifiers; \`RUST_RESERVED_KEYWORDS\` disambiguates. Confirmed by direct instrumentation of \`rustFiniteManifestTargets\`: the \`if !(page.width > 0.0 && …)\` of the \`stdio.pdf\` \`✳️x\` schema ancestor was tripping the downstream \`/[#!]/\` scan and disqualifying an otherwise-trusted ancestor.
 *
 * \`!=\` is one punctuation token (see the \`punctuation\` list of \`rustTokens\`), never a macro bang, yet it literally CONTAINS a \`!\` character, which the downstream \`/[#!]/\` scan matches as a textual substring regardless of token boundaries. It is the same false-positive family as the bare-\`!\` negation (the \`d.code.0 != CODE_TEXT_EMPTY\` of the \`stdio.pdf\` \`✳️a\` schema ancestor was tripping it after the bare-\`!\` fix) and is neutralized the same way: comparison operators are expression-position only and can never expand into, or hide, a \`mod\`. */
function rustCodeOnlyTextForMacroTrust(source: string): string {`);
  edit(F.graph, `export function rustManifestReferenceTokens(`, `/** 🧷️ Builds the manifest-relative reference tokens of one Rust source.
 *
 * Two structurally distinct failures are kept apart in the unsupported reason: \`targets\` is \`undefined\` when a reference never reached the per-candidate map of \`rustFiniteManifestTargets\` at all (an early whole-file guard bailed, or its own try/catch threw for this candidate specifically), so no proof was ever attempted; \`targets.length !== 1\` with \`targets\` DEFINED means the proof ran to completion and found zero or several distinct physical targets, a genuine ambiguity and not a missing proof. Three earlier diagnoses mistook the first for the second (see the \`rust-path-join\` ticket history).
 *
 * \`env::temp_dir()\`, \`env::var[_os]()\`, \`env::args()\` or a bare \`fn\` parameter with no local \`CARGO_MANIFEST_DIR\` anchor PROVES the join's root can never name a repository file at plan time: that is a proven non-reference, so it is never recorded at all (not merely marked resolved). An unrecognized base is untouched and still falls through to the \`rust-path-join-unproven\` rows unchanged. */
export function rustManifestReferenceTokens(`);
  return counts;
}
