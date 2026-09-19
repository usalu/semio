import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { produceFreshComponentV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";
import { type FreshBuildControlV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts";
import { compileGisScopeExport } from "../../🧬️schema/🟦️.ts";

const GIS_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json";

/** 🌉️ Proves the receipt-bound GIS cold-pair, tiled-map patch, and addressed mutation corpus. */
export async function proveGisComponentColdMapPatch(repoRoot: string): Promise<void> {
  const fixtureRoot = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🌉️component-cold-map-patch");
  const testRoot = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch");
  const fixtureBytes = readFileSync(join(fixtureRoot, "🔣️.json"));
  const fixture = JSON.parse(fixtureBytes.toString("utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisComponentColdMapPatch");
  if (!validate(fixture)) throw new Error(`invalid GIS component cold-map corpus: ${JSON.stringify(validate.errors)}`);
  const nodeHash = createHash("sha256").update(fixtureBytes).digest("hex");
  const webHash = Buffer.from(await crypto.subtle.digest("SHA-256", fixtureBytes)).toString("hex");
  if (nodeHash !== webHash || /^0{64}$/u.test(nodeHash)) throw new Error("GIS component corpus SHA-256 oracle mismatch");
  const owner = structuredClone(fixture);
  const admitted = (candidate: any): boolean =>
    validate(candidate) &&
    candidate.package.pluginId === "gis" &&
    candidate.package.packageId === "semio:gis" &&
    candidate.package.cargoPackage === "semio-s-plugin-gis" &&
    candidate.package.componentFile === "semio_s_plugin_gis.wasm" &&
    candidate.app.id === "s.gis.gismap@1/*#editor" &&
    candidate.app.actionId === "patchPositions" &&
    candidate.surface === "1:gis2d.play.composite" &&
    candidate.authority.activationGeneration > 0 &&
    candidate.authority.transferGeneration > 0 &&
    candidate.receipt.componentSha256 === owner.receipt.componentSha256 &&
    candidate.receipt.descriptorSha256 === owner.receipt.descriptorSha256;
  if (!admitted(fixture)) throw new Error("GIS component cold-map owner was denied");
  for (const hostile of fixture.hostile) {
    const candidate = structuredClone(fixture);
    switch (hostile) {
      case "foreign-package":
        candidate.package.packageId = "semio:stdio";
        break;
      case "stale-lifetime":
        candidate.authority.activationGeneration = 0;
        break;
      case "wrong-descriptor":
        candidate.receipt.descriptorSha256 = "00".repeat(32);
        break;
      case "wrong-action-owner":
        candidate.app.actionId = "replacePositions";
        break;
      case "changed-component-after-receipt":
        candidate.receipt.componentSha256 = "44".repeat(32);
        break;
      default:
        throw new Error(`unhandled GIS component hostile ${hostile}`);
    }
    if (admitted(candidate)) throw new Error(`GIS component hostile admitted ${hostile}`);
  }
  const testSource = readFileSync(join(testRoot, "🦀️.rs"), "utf8");
  for (const marker of [
    "ColdDocumentPairPage",
    "ColdPairIngressStatus::Applied",
    "TiledMapScene",
    "UiTurnPatchTransportLease",
    "ActionInvocation",
    "\"patchPositions\"",
    "Event::PatchAck",
    "SEMIO_GIS_COMPONENT_SHA256",
    "SEMIO_GIS_DESCRIPTOR_SHA256",
  ]) {
    if (!testSource.includes(marker)) throw new Error(`GIS component acceptance omits ${marker}`);
  }
  const manifest = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml"), "utf8");
  if (!manifest.includes('name = "component_cold_map_patch"') || !manifest.includes("semio-framework-plugin-host") || !manifest.includes("semio-framework-ui-scene")) {
    throw new Error("GIS component acceptance is not mounted with its exact first-party host and scene owners");
  }
  console.log(`gis-component-cold-map-patch-source: AJV=1 SHA256=node+webcrypto hostile=${fixture.hostile.length} markers=9; no GIS component build or browser acceptance claim`);
}

function freshGisComponentBuildControl(): { readonly control: FreshBuildControlV1; close(): void } {
  const duration = Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 86_400_000);
  if (!Number.isSafeInteger(duration) || duration < 60_000 || duration > 86_400_000) throw new Error("GIS component build budget must be between one minute and 24 hours");
  const deadline = Date.now() + duration;
  let interrupted = false;
  const interrupt = () => {
    interrupted = true;
  };
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", interrupt);
  return {
    control: Object.freeze({
      diagnosticsRoot: undefined,
      cancelled: () => interrupted,
      remainingMs: () => Math.max(0, deadline - Date.now()),
      checkpoint(stage, completed, total) {
        if (interrupted || Date.now() >= deadline) throw new Error(`GIS component production cancelled during ${stage}`);
        console.log(`gis-component-production: ${stage} ${completed}/${total}`);
      },
    }),
    close() {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", interrupt);
    },
  };
}

export class ComponentColdMapPatchCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisComponentColdMapPatch(this.repoRoot);
  }
}

export class ComponentColdMapPatchNativeCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisComponentColdMapPatch(this.repoRoot);
    const artifactRoot = join(this.root, "dist/component-cold-map-patch-native-check");
    rmSync(artifactRoot, { recursive: true, force: true });
    mkdirSync(artifactRoot, { recursive: true, mode: 0o700 });
    const runRoot = mkdtempSync(join(artifactRoot, "gis-component-cold-map-patch-"));
    const target = join(runRoot, "producer-target");
    const stage = join(runRoot, "stage");
    mkdirSync(target, { mode: 0o700 });
    mkdirSync(stage, { mode: 0o700 });
    const build = freshGisComponentBuildControl();
    const control = build.control;
    try {
      const produced = await produceFreshComponentV1(
        this.repoRoot,
        {
          pluginId: "gis",
          cargoPackage: "semio-s-plugin-gis",
          componentPackageId: "semio:gis",
          outputName: "semio_s_plugin_gis.wasm",
          componentProfile: "wasm-release",
          rootCdylib: true,
        },
        target,
        stage,
        control,
        (lease) =>
          lease.consume(async (component) => {
            const componentSha256 = createHash("sha256").update(component).digest("hex");
            const stagedComponent = readFileSync(join(stage, "component.wasm"));
            const descriptor = readFileSync(join(stage, "descriptor.semio"));
            const descriptorSha256 = createHash("sha256").update(descriptor).digest("hex");
            if (createHash("sha256").update(stagedComponent).digest("hex") !== componentSha256) throw new Error("leased GIS component differs from staged component");
            const receipts = await runExactCargoLaws({
              cwd: this.repoRoot,
              groups: [
                {
                  package: "semio-s-plugin-gis",
                  target: { kind: "test", name: "component_cold_map_patch" },
                  cargoArgs: ["--no-default-features"],
                  laws: [
                    "genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface",
                    "genuine_gis_component_rejects_stale_cold_authority_before_loading",
                  ],
                },
              ],
              artifactDir: join(artifactRoot, "exact"),
              env: {
                ...process.env,
                SEMIO_GIS_COMPONENT_WASM: join(stage, "component.wasm"),
                SEMIO_GIS_COMPONENT_SHA256: componentSha256,
                SEMIO_GIS_DESCRIPTOR_SHA256: descriptorSha256,
              },
              nativeEnv: { RUST_MIN_STACK: process.env.SEMIO_TEST_NATIVE_RUST_MIN_STACK ?? "268435456" },
              buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 86_400_000),
              listBudgetMs: 120_000,
              lawBudgetMs: 300_000,
              cancelled: control.cancelled,
              progress(event) {
                console.log(`gis-component-cold-map-patch ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`);
              },
            });
            return { componentSha256, descriptorSha256, receipts };
          }),
      );
      if (
        produced.receipt.component.sha256 !== produced.derived.componentSha256 ||
        produced.receipt.descriptor.sha256 !== produced.derived.descriptorSha256 ||
        produced.receipt.pluginId !== "gis" ||
        produced.receipt.packageId !== "semio:gis"
      ) {
        throw new Error("GIS component native acceptance differs from its fresh producer receipt");
      }
      for (const receipt of produced.derived.receipts) console.log(`gis-component-cold-map-patch-receipt: ${JSON.stringify(receipt)}`);
      console.log(
        `gis-component-cold-map-patch-native: component=${produced.receipt.component.sha256} descriptor=${produced.receipt.descriptor.sha256} exact=2 stage=${stage}; genuine Wasmtime GIS boundary only, Hub selection and Shell browser acceptance remain separate`,
      );
    } finally {
      build.close();
      rmSync(runRoot, { recursive: true, force: true });
    }
  }
}
