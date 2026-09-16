import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { appBreadcrumb, resolveAppBreadcrumb } from "@semio-tech/framework-renderer-react";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");

/** @emoji 🏷️ Demonstrator pane shells must resolve reuse terminology to Entwerfen-mit-Bestand chrome, never `semio`. */
const DEMONSTRATOR_PANE_NAVBAR_LABELS: Record<string, string> = {
  "s.procedural.generation3d@1/*#editor": "Entwerfen mit Bestand · Generator",
  "s.cad.cad@1/*#editor": "Entwerfen mit Bestand · Koordinator",
  "s.puzzle.puzzle3d@1/*#editor": "Entwerfen mit Bestand · Aggregator",
  "s.energy.model@1/*#editor": "Entwerfen mit Bestand · Energie",
  "s.sourcing.curation@1/*#editor": "Entwerfen mit Bestand · Aussuchen",
  "s.process.process3d@1/*#editor": "Entwerfen mit Bestand · Bearbeiten",
  "s.gis.gismap@1/*#editor": "Entwerfen mit Bestand · Verfolgen",
  "s.fem.fem3d@1/*#editor": "Entwerfen mit Bestand · Statik",
};

const MANIFEST_SOURCES: Record<string, string> = {
  "s.procedural.generation3d@1/*#editor": "✏️s/🔌️plugins/🎪️demonstrator/🔣️.json",
  "s.cad.cad@1/*#editor": "✏️s/🔌️plugins/🎪️demonstrator/🔣️.json",
  "s.puzzle.puzzle3d@1/*#editor": "✏️s/🔌️plugins/🎪️demonstrator/🔣️.json",
  "s.sourcing.curation@1/*#editor": "✏️s/🔌️plugins/🎪️demonstrator/🔣️.json",
  "s.process.process3d@1/*#editor": "✏️s/🔌️plugins/🎪️demonstrator/🔣️.json",
  "s.gis.gismap@1/*#editor": "✏️s/🔌️plugins/🎪️demonstrator/🔣️.json",
  "s.energy.model@1/*#editor": "✏️s/🔌️plugins/🔋️energy/🔣️.json",
  "s.fem.fem3d@1/*#editor": "✏️s/🔌️plugins/🏗️fem/🔣️.json",
};

type ManifestApp = {
  readonly controllerId?: string;
  readonly id?: string;
  readonly breadcrumb?: readonly string[];
  readonly terminologyBreadcrumbs?: Readonly<Record<string, readonly string[]>>;
};

function loadApps(manifestPath: string): readonly ManifestApp[] {
  const parsed = JSON.parse(readFileSync(resolve(repoRoot, manifestPath), "utf8")) as {
    manifest?: { apps?: ManifestApp[] };
    apps?: ManifestApp[];
  };
  return parsed.manifest?.apps ?? parsed.apps ?? [];
}

function findApp(apps: readonly ManifestApp[], controllerId: string): ManifestApp | undefined {
  return apps.find((app) => (app.controllerId ?? app.id) === controllerId);
}

export async function registerDemonstratorPaneBrandingTests(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, expect, it } = vitest;

  describe("demonstrator pane navbar labels (reuse terminology)", () => {
    for (const [controllerId, expectedLabel] of Object.entries(DEMONSTRATOR_PANE_NAVBAR_LABELS)) {
      it(`${controllerId} → ${expectedLabel}`, () => {
        const manifestPath = MANIFEST_SOURCES[controllerId];
        const app = findApp(loadApps(manifestPath), controllerId);
        expect(app, `missing app in ${manifestPath}`).toBeTruthy();
        const resolved = appBreadcrumb(resolveAppBreadcrumb(app!, "reuse"));
        expect(resolved).toBe(expectedLabel);
        expect(resolved.toLowerCase()).not.toContain("semio");
      });
    }
  });
}

if (import.meta.vitest) {
  await registerDemonstratorPaneBrandingTests(import.meta.vitest);
}
