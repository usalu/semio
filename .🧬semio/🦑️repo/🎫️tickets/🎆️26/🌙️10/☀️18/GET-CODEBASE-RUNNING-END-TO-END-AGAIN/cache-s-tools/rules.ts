/** 🧭 Cache decisions for ✏️s targets: names whose cache stays off carry the reason; every other cache-off deterministic or unclassified target is flipped on. */
export const KEEP: { readonly pattern: RegExp; readonly reason: string }[] = [
  { pattern: /^(live-agent-loop-check|hub-agent-participant-check|plugin-coverage-check|user-path-check|inference-quartet-check|security-check|agent-reply-check)$/u, reason: "live hub or serve endpoints, hour-scale budgets" },
  { pattern: /^(fixture-ownership-check|canonical-pair-composition-check|rebuild-all)$/u, reason: "orchestrates other tasks with --skip-cache or rebuilds everything" },
  { pattern: /^guest-framework-check$/u, reason: "plan-defined multi-workspace cargo check; input closure not statically declarable" },
  { pattern: /^browser-test$/u, reason: "drives a real browser" },
  { pattern: /^canonical-architecture$/u, reason: "repo-wide architecture gate run with --skip-cache; pinned off by the cache policy targetDefaults" },
  { pattern: /^tiles-prefetch$/u, reason: "network download into an ignored directory" },
  { pattern: /^(oracle-[a-z]+|deps|oracle-setup)$/u, reason: "external EnergyPlus toolchain and uv environment" },
  { pattern: /^lock$/u, reason: "cargo update: network and mutates Cargo.lock" },
  { pattern: /^run$/u, reason: "interactive shell" },
  { pattern: /^(composition-prepare)$/u, reason: "writes capability links into the source tree" },
  { pattern: /^deletion-proof$/u, reason: "requires an externally provided SEMIO_TEST_ARTIFACT_DIR" },
  { pattern: /^sqlite-oracle$/u, reason: "argument and stdin driven helper, no standalone invocation" },
  { pattern: /^(regenerate-example-assets|test-bestest-regenerate|fixture|generate-register-catalog)$/u, reason: "regenerates checked-in files at undeclared paths" },
  { pattern: /^(mcp-client-e2e|verify-fem-live-visual-publication)$/u, reason: "live end-to-end" },
  { pattern: /^(dev|serve|watch|start)$/u, reason: "continuous" },
  { pattern: /^(install|setup|publish|deploy|clean|seed|migrate|update|purge|fuzz|new)$/u, reason: "side effecting" },
];
export const keepReason = (name: string): string | undefined => KEEP.find((rule) => rule.pattern.test(name))?.reason;
