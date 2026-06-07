#!/usr/bin/env bun
/** 🧭 `@gis/map/play` task router: `bun ./script.ts <dev|build|test|tiles|fixture>`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import {
  GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
  GIS_MAP_PREFETCH_RASTER_Z_MAX,
  GIS_MAP_VECTOR_TILE_MAX_Z,
  prefetchMapTiles,
  type GisMapPrefetchBounds,
} from "../../../ui/styling/vite-elements-assets.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../../ui/styling/playground-dev-ports.ts";
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBun,
  runBundleScriptMain,
  runCargo,
  runViteBunxDev,
  runVitest,
} from "../../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../rs/script.ts");
const graphFixturePath = join(import.meta.dir, "../fixture/reuse.graph.gis.json");
const mapFixturePath = join(import.meta.dir, "../fixture/reuse.map.gis.json");

interface RawGeo {
  readonly latitude?: number;
  readonly longitude?: number;
  readonly source_url?: string;
}

interface RawProjekt {
  readonly id: string;
  readonly name?: string;
  readonly geo?: RawGeo;
}

interface RawBauwerk {
  readonly id: string;
  readonly name?: string;
  readonly geo?: { readonly donor?: RawGeo | null; readonly receiver?: RawGeo | null };
}

interface RawReuseChain {
  readonly bauteilgruppe_id: string;
  readonly bauteilgruppe_name?: string;
  readonly donor_bauwerk_ids?: readonly string[];
  readonly receiver_projekt_id?: string;
  readonly donor_coordinates?: readonly { readonly latitude: number; readonly longitude: number }[];
  readonly receiver_coordinates?: { readonly latitude: number; readonly longitude: number } | null;
}

interface RawReuseGraph {
  readonly nodes?: {
    readonly projekte?: readonly RawProjekt[];
    readonly bauwerke?: readonly RawBauwerk[];
  };
  readonly reuse_chains?: readonly RawReuseChain[];
}

interface GisMapFixturePositionV1 {
  readonly id: string;
  readonly lon: number;
  readonly lat: number;
  readonly label: string;
  readonly name: string;
  readonly kind: "receiver" | "donor";
  readonly icon: "landmark" | "box";
  readonly sourceUrl?: string;
}

interface GisMapFixtureRouteV1 {
  readonly id: string;
  readonly points: readonly (readonly [number, number])[];
  readonly kind: "reuse";
  readonly label?: string;
}

interface GisMapFixtureV1 {
  readonly schema: "gis.map.fixture/v1";
  readonly name: string;
  readonly positions: readonly GisMapFixturePositionV1[];
  readonly routes: readonly GisMapFixtureRouteV1[];
  readonly regions: readonly [];
}

function isHttpSourceUrl(value: string | undefined): value is string {
  return typeof value === "string" && value.startsWith("http");
}

function buildReuseMapFixture(graph: RawReuseGraph): GisMapFixtureV1 {
  const projekte = new Map((graph.nodes?.projekte ?? []).map((row) => [row.id, row]));
  const bauwerke = new Map((graph.nodes?.bauwerke ?? []).map((row) => [row.id, row]));
  const positions = new Map<string, GisMapFixturePositionV1>();
  const routes: GisMapFixtureRouteV1[] = [];

  for (const chain of graph.reuse_chains ?? []) {
    const receiver = chain.receiver_coordinates;
    const receiverProjekt = chain.receiver_projekt_id ? projekte.get(chain.receiver_projekt_id) : undefined;
    const receiverName = receiverProjekt?.name ?? chain.receiver_projekt_id ?? "Receiver";
    const receiverSource = isHttpSourceUrl(receiverProjekt?.geo?.source_url) ? receiverProjekt.geo.source_url : undefined;

    if (receiver && chain.receiver_projekt_id) {
      const id = chain.receiver_projekt_id;
      positions.set(id, {
        id,
        lon: receiver.longitude,
        lat: receiver.latitude,
        label: receiverName,
        name: receiverName,
        kind: "receiver",
        icon: "landmark",
        ...(receiverSource ? { sourceUrl: receiverSource } : {}),
      });
    }

    const donorIds = chain.donor_bauwerk_ids ?? [];
    const donorCoords = chain.donor_coordinates ?? [];
    for (let i = 0; i < donorCoords.length; i += 1) {
      const coord = donorCoords[i];
      const donorId = donorIds[i] ?? donorIds[0] ?? `donor_${chain.bauteilgruppe_id}_${i}`;
      const bauwerk = bauwerke.get(donorId);
      const donorName = bauwerk?.name ?? donorId;
      const donorSource = isHttpSourceUrl(bauwerk?.geo?.donor?.source_url) ? bauwerk.geo.donor.source_url : undefined;
      positions.set(donorId, {
        id: donorId,
        lon: coord.longitude,
        lat: coord.latitude,
        label: donorName,
        name: donorName,
        kind: "donor",
        icon: "box",
        ...(donorSource ? { sourceUrl: donorSource } : {}),
      });

      if (receiver) {
        routes.push({
          id: `${chain.bauteilgruppe_id}:${donorId}:${i}`,
          points: [
            [coord.longitude, coord.latitude],
            [receiver.longitude, receiver.latitude],
          ],
          kind: "reuse",
          ...(chain.bauteilgruppe_name ? { label: chain.bauteilgruppe_name } : {}),
        });
      }
    }
  }

  return {
    schema: "gis.map.fixture/v1",
    name: "Reuse map",
    positions: [...positions.values()],
    routes,
    regions: [],
  };
}

class FixtureScript extends BundleScript {
  run(): void {
    const raw = JSON.parse(readFileSync(graphFixturePath, "utf8")) as RawReuseGraph;
    const fixture = buildReuseMapFixture(raw);
    writeFileSync(mapFixturePath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
    console.log(
      `[DEBUG] gis map fixture: ${fixture.positions.length} positions, ${fixture.routes.length} routes -> ${mapFixturePath}`,
    );
  }
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runViteBunxDev(this.root, segments, {
      portEnv: playgroundPortEnv("gis-map"),
      defaultPort: playgroundDevPortString("gis-map"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "gis_map"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runVitest(this.root, segments);
  }
}

function parseFlagInt(segments: string[], name: string): number | undefined {
  const prefix = `--${name}=`;
  const hit = segments.find((s) => s.startsWith(prefix));
  if (!hit) {
    return undefined;
  }
  const n = Number(hit.slice(prefix.length));
  return Number.isFinite(n) ? Math.floor(n) : undefined;
}

function parsePrefetchBounds(segments: string[]): GisMapPrefetchBounds {
  const num = (key: string, fallback: number) => parseFlagInt(segments, key) ?? fallback;
  return {
    west: num("west", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.west),
    south: num("south", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.south),
    east: num("east", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.east),
    north: num("north", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.north),
  };
}

class TilesScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const rasterOnly = segments.includes("--raster-only");
    const vectorOnly = segments.includes("--vector-only");
    const fullRaster = segments.includes("--full-raster");
    const zMinRaster = parseFlagInt(segments, "raster-z-min") ?? 0;
    const zMaxRaster = parseFlagInt(segments, "raster-z-max") ?? GIS_MAP_PREFETCH_RASTER_Z_MAX;
    const zMinVector = parseFlagInt(segments, "vector-z-min") ?? 0;
    const zMaxVector = parseFlagInt(segments, "vector-z-max") ?? GIS_MAP_VECTOR_TILE_MAX_Z;
    const concurrency = parseFlagInt(segments, "concurrency") ?? 4;
    await prefetchMapTiles({
      repoRoot: this.repoRoot,
      bounds: parsePrefetchBounds(segments),
      raster: !vectorOnly,
      vector: !rasterOnly,
      zMinRaster,
      zMaxRaster,
      zMinVector,
      zMaxVector,
      concurrency,
    });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("tiles", TilesScript)
  .register("fixture", FixtureScript);

await runBundleScriptMain(router, import.meta.url);
