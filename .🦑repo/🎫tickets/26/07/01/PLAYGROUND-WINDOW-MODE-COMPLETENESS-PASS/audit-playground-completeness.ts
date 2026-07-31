/** @emoji 🧪 Playground window/mode completeness audit helpers. */
import { CommandBus, type AppTools, type BaseModeRuntime, type ToolNode } from "@semio-tech/framework-core";

export interface PlaygroundAuditTarget {
  readonly id: string;
  readonly modes: readonly BaseModeRuntime[];
}

function countToolLeaves(tools: AppTools | undefined): number {
  if (!tools?.length) return 0;
  const walk = (nodes: readonly ToolNode[]): number =>
    nodes.reduce((count, node) => {
      if (node.kind === "separator") return count;
      if (node.kind === "collection") return count + walk(node.children);
      return count + 1;
    }, 0);
  return walk(tools);
}

/** @emoji ✅ Returns human-readable failures for incomplete playground shells. */
export function auditPlaygroundTarget(target: PlaygroundAuditTarget): string[] {
  const failures: string[] = [];
  for (const mode of target.modes) {
    if (countToolLeaves(mode.tools) === 0) {
      failures.push(`${target.id} mode "${mode.id}" missing footer tools`);
    }
    for (const windowKind of mode.windowKinds) {
      if (!windowKind.measures?.length) {
        failures.push(`${target.id} mode "${mode.id}" window "${windowKind.id}" missing measures`);
      }
      if (!windowKind.engagement) {
        failures.push(`${target.id} mode "${mode.id}" window "${windowKind.id}" missing engagement`);
      }
    }
  }
  return failures;
}

type ControllerFactory = (bus: CommandBus, notify: () => void) => { readonly mainMode: BaseModeRuntime; readonly generateMode?: BaseModeRuntime };

/** @emoji 🔎 Audits every known playground controller shell. */
export async function auditAllPlaygrounds(): Promise<string[]> {
  const bus = new CommandBus();
  const notify = () => {};
  const targets: PlaygroundAuditTarget[] = [];

  const register = (id: string, factory: ControllerFactory, includeGenerate = false) => {
    const ctrl = factory(bus, notify);
    const modes = includeGenerate && ctrl.generateMode ? [ctrl.mainMode, ctrl.generateMode] : [ctrl.mainMode];
    targets.push({ id, modes });
  };

  const { DrawPlayController } = await import("@semio-tech/draw-play");
  register("draw", (b, n) => new DrawPlayController(b, n));

  const { FlowPlayController } = await import("@semio-tech/flow-play");
  register("flow", (b, n) => new FlowPlayController(b, n), true);

  const { FormsPlayController } = await import("@semio-tech/forms-play");
  register("forms", (b, n) => new FormsPlayController(b, n));

  const { RasterPlayController } = await import("@semio-tech/raster-play");
  register("raster", (b, n) => new RasterPlayController(b, n));

  const { WriterPlayController } = await import("@semio-tech/writer-play");
  const { createWriterDocument } = await import("@semio-tech/writer-core");
  register("writer", (b, n) => new WriterPlayController(b, n, JSON.stringify(createWriterDocument({ id: "audit", languageId: "jack" }))));

  const { SemiosPlayController } = await import("@semio-tech/semios-play");
  register("semios", (b, n) => new SemiosPlayController(b, n));

  const { ShootingPlayController } = await import("@semio-tech/shooting-play");
  register("shooting", (b, n) => new ShootingPlayController(b, n));

  const { MapPlayController } = await import("@semio-tech/gis-2d-play");
  register("gis/2d", (b, n) => new MapPlayController(b, n));

  const { Procedural2dPlayController } = await import("@semio-tech/procedural-2d-play");
  register("procedural/2d", (b, n) => new Procedural2dPlayController(b, n), true);

  const { ProceduralPlayController } = await import("@semio-tech/procedural-3d-play");
  register("procedural/3d", (b, n) => new ProceduralPlayController(b, n), true);

  const { Puzzle2dPlayShellController } = await import("@semio-tech/puzzle-2d-play");
  register("puzzle/2d", (b, n) => new Puzzle2dPlayShellController(b, n));

  const { Puzzle3dPlayShellController } = await import("@semio-tech/puzzle-3d-play");
  register("puzzle/3d", (b, n) => new Puzzle3dPlayShellController(b, n));

  const { Puzzle5dPlayShellController } = await import("@semio-tech/puzzle-5d-play");
  register("puzzle/5d", (b, n) => new Puzzle5dPlayShellController(b, n));

  const { DagPlayController } = await import("@semio-tech/dag-play");
  register("mathematical/dag", (b, n) => new DagPlayController(b, n));

  const { TrinityJackPlayController } = await import("@semio-tech/trinity-jack-play");
  register("trinity/jack", (b, n) => new TrinityJackPlayController(b, n));

  const { TrinityRewritePlayController } = await import("@semio-tech/trinity-rewrite-play");
  register("trinity/rewrite", (b, n) => new TrinityRewritePlayController(b, n));

  const { PresentationPlayController } = await import("@semio-tech/framework-presentation-play");
  register("framework/presentation", (b, n) => new PresentationPlayController(b, n));

  return targets.flatMap((target) => auditPlaygroundTarget(target));
}
