import { DEFAULT_VOXEL_BRUSH_DIMENSIONS, snapCadToVoxelCenter, voxelGridKey } from "../../../../../../puzzle/3d/react/index.tsx";

const scale = DEFAULT_VOXEL_BRUSH_DIMENSIONS;
const painted: string[] = [];
let lastCommitKey: string | null = null;

const commitAt = (cad: readonly [number, number, number]): void => {
  const key = voxelGridKey(cad, scale);
  if (lastCommitKey === key) {
    return;
  }
  lastCommitKey = key;
  painted.push(key);
  console.log("[DEBUG] puzzle3d voxel brush commit", snapCadToVoxelCenter(cad, scale), scale);
};

const onMove = (cad: readonly [number, number, number], altKey: boolean): void => {
  if (altKey) {
    commitAt(cad);
  }
};

onMove([0.2, 0.2, 0.2], true);
onMove([0.2, 0.2, 0.2], true);
onMove([10.2, 0.2, 0.2], true);
onMove([20.2, 0.2, 0.2], true);
onMove([20.2, 0.2, 0.2], true);
onMove([30.2, 0.2, 0.2], false);

if (painted.length !== 3) {
  console.error(`expected 3 unique cells, got ${painted.length}`, painted);
  process.exit(1);
}

console.log("[DEBUG] alt-drag paint trail ok", painted);
