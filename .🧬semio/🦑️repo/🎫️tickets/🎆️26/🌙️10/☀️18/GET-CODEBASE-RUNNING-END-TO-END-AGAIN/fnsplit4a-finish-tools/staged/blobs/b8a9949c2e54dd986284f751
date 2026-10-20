import { buildTokensState } from "./🔶️tokens/🟦️.ts";
import { buildResolveQualifiedFunctionState } from "./🔸️resolve-qualified-function/🟦️.ts";
import { buildVisit } from "./🟠️visit/🟦️.ts";

/** 🚱️ Proves a `.join()` call chain's ROOT can never resolve to a manifest-relative path at plan
 * time — `env::temp_dir()`, an `env::var`/`env::var_os` runtime lookup (optionally routed through
 * `std::env::args()`-style CLI access), or a bare `fn` parameter this file never itself binds to a
 * `CARGO_MANIFEST_DIR` proof — so literal segments joined onto it name a freshly synthesized runtime
 * path, not a rewritable repository reference. Distinguishes PROVEN-non-repo from merely-unproven:
 * an unrecognized root is left alone so the caller still treats it as unresolved. One bounded
 * same-file helper-function hop (depth 1) lets a `fn returning_a_path() -> PathBuf { ... }` whose own
 * tail expression is proven non-repo count for its callers too, without unbounded recursion. */
export function inspectRustNonRepoJoinBaseSpans(source: string): ReadonlySet<number> {
  if (!/\.\s*join\s*\(/u.test(source)) return new Set();
  const { tokens, pairs, rootEnd, literal, walkJoinChain, rows } = buildTokensState({ source });
  const { tupleCallNonRepoPositions } = buildResolveQualifiedFunctionState({ tokens, pairs, rootEnd, literal });
  buildVisit({ tokens, pairs, tupleCallNonRepoPositions, rootEnd, walkJoinChain });
  return rows;
}
