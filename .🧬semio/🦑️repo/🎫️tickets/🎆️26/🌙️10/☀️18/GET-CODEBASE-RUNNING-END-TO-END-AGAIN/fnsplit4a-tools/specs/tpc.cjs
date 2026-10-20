const V = "️";
const g = (e, d, name, fe, doc, fdoc, from, to) => ({ dir: e + V + d, emoji: fe, doc, name, fdoc, from, to });
const spec = {
  file: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🟦️.ts", method: "run", className: "TestScript", keepFrom: 907,
  context: { dir: "🧭" + V + "route-context", emoji: "🧭", name: "TestRouteContext", paramName: "context", doc: "The invocation, roots and re-entry of the test script that its route group functions receive instead of the script instance.",
    fields: [{ name: "invocation", type: "ScriptInvocation" }, { name: "repoRoot", type: "string" }, { name: "root", type: "string" }, { name: "run", type: "(segments: string[]) => Promise<void>" }],
    importLines: ['import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";'],
    thisFields: ["invocation", "repoRoot", "root", "run"], init: "{ invocation: this.invocation, repoRoot: this.repoRoot, root: this.root, run: (routeSegments) => this.run(routeSegments) }" },
  groups: [
    g("🧷", "fixture-and-nx-routes", "runFixtureAndNxRoutes", "🪡", "The fixture placement, original control, target execution and Nx owner routes of the test script.", "Runs the fixture placement, original control, target execution and Nx owner routes; true when one matched.", 18, 63),
    g("🛤", "runtime-and-path-routes", "runRuntimeAndPathRoutes", "🛣", "The runtime actor, artifact command, Cargo search path, Nx inference and native source routes of the test script.", "Runs the generator, runtime, artifact, Cargo, build discovery, Nx inference and ownership routes; true when one matched.", 65, 166),
    g("🏗", "build-and-direction-routes", "runBuildAndDirectionRoutes", "🧱", "The build surface, dependency direction and canonical execution routes of the test script.", "Runs the build surface, native dependency, canonical JSON, mutation authority, direction and corpus routes; true when one matched.", 171, 285),
    g("🗺", "source-topology-routes", "runSourceTopologyRoutes", "📍", "The Go dispatch, source topology and root source law routes of the test script.", "Runs the runtime corpus, Go dispatch, catalog, topology, platform and root source routes; true when one matched.", 291, 405),
    g("⚖", "ownership-law-routes", "runOwnershipLawRoutes", "🔏", "The crate tree, family, publication and ownership law routes of the test script.", "Runs the crate tree, mutation family, publication, configuration and source ownership routes; true when one matched.", 411, 556),
    g("🔗", "reference-authority-routes", "runReferenceAuthorityRoutes", "🪝", "The path statute, mutation source, reference and authority routes of the test script.", "Runs the path statute, mutation source, provider, reference and recovery authority routes; true when one matched.", 562, 738),
    g("📖", "readme-and-mutation-routes", "runReadmeAndMutationRoutes", "🧬", "The README source, divergence callback, verification and mutation witness routes of the test script.", "Runs the README authority, divergence callback, source service, regeneration and mutation witness routes; true when one matched.", 743, 901),
  ],
};
require("fs").writeFileSync(process.argv[2], JSON.stringify(spec, null, 1));
