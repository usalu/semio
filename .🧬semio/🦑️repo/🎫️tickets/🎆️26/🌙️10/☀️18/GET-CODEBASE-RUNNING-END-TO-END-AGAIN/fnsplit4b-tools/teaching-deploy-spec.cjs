const { R, g, dir, doc, write } = require("./lib_ks.cjs");
const file = R + "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🟦️.ts";
write({
  file, describeLine: 114,
  shared: { dir: dir("🧰", "deploy-fixtures"), doc: doc("🧰", "The shared deployment test fixtures: the course locations, the parsed compose and workflow documents and the sealed document builders.") },
  sharedRewrite: [['import.meta.url)), "../..")', 'import.meta.url)), "../../..")']],
  parts: [
    g("🛑", "cancellation-and-staging", "registerCancellationAndStagingTests", "🧹", "The deployment cancellation, release build and site staging tests.", "Registers the preflight alignment, parent cancellation, release build, site staging and staging refusal tests.", 115, 181),
    g("🐳", "docker-context-and-image", "registerDockerContextAndImageTests", "📦", "The docker context, apex domain, schema contract, drift and proctor image tests.", "Registers the docker context, apex domain, schema contract, source drift, image gate and runtime role tests.", 188, 259),
    g("🧩", "compose-caddy-and-workflow", "registerComposeCaddyAndWorkflowTests", "🔐", "The compose stack, Caddy proxy and release workflow tests.", "Registers the hardened compose stack, the Caddy proxy and the release workflow tests.", 270, 320),
    g("🌐", "sealed-publication-and-bundle", "registerSealedPublicationAndBundleTests", "🚛", "The sealed document, CDN artifact, stack bundle, readiness gate and response header tests.", "Registers the sealed document, CDN artifact, stack bundle, readiness gate and response header tests.", 357, 471),
  ],
}, process.argv[2]);
