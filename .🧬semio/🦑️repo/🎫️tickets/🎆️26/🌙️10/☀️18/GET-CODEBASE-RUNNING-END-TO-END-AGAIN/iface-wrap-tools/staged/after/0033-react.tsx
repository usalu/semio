/** ☁️ World3d instance layers, point cloud: the point cloud layer visual and the layer that draws it. */
import { useEffect, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { BufferAttribute, BufferGeometry, PointsMaterial } from "@semio-tech/ui-react";
import { base64ToBytes } from "../../../🖌️Paint2dHost/🟦️.tsx";
import { type WorldPointCloudLayerRecord } from "../../🔭️scene-schema/🟦️.ts";
import { parseJsonArray } from "../../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldPointCloudLayer
/** ☁️ Renders `World3dScene.pointsJson` layers as GPU point sprites — decodes each layer's base64
 * position/color buffers into a `BufferGeometry` and draws it with a `PointsMaterial`, mounted
 * alongside `🗺️WorldTerrainLayer` in the `World3dHost` scene tree. */
type WorldPointCloudLayerVisual = { readonly geometry: BufferGeometry; readonly material: PointsMaterial };

function pointCloudLayerVisual(layer: WorldPointCloudLayerRecord): WorldPointCloudLayerVisual {
  const geometry = new BufferGeometry();
  const positionBytes = base64ToBytes(layer.positionsB64);
  const positions = new Float32Array(positionBytes.buffer, positionBytes.byteOffset, positionBytes.byteLength / Float32Array.BYTES_PER_ELEMENT);
  geometry.setAttribute("position", new BufferAttribute(positions, 3));
  const hasColors = Boolean(layer.colorsB64);
  if (layer.colorsB64) geometry.setAttribute("color", new BufferAttribute(base64ToBytes(layer.colorsB64), 3, true));
  const material = new PointsMaterial({ size: layer.size, sizeAttenuation: layer.sizeAttenuation, vertexColors: hasColors });
  return { geometry, material };
}

export function WorldPointCloudLayer({ pointsJson }: { readonly pointsJson: string | undefined }) {
  const layers = useMemo(() => parseJsonArray<WorldPointCloudLayerRecord>(pointsJson), [pointsJson]);
  const visuals = useMemo(() => {
    const map = new Map<string, WorldPointCloudLayerVisual>();
    for (const layer of layers) map.set(layer.id, pointCloudLayerVisual(layer));
    return map;
  }, [layers]);

  useEffect(() => {
    return () => {
      for (const visual of visuals.values()) {
        visual.geometry.dispose();
        visual.material.dispose();
      }
    };
  }, [visuals]);

  if (layers.length === 0) return null;

  return (
    <group>
      {layers.map((layer) => {
        const visual = visuals.get(layer.id);
        if (!visual) return null;
        return <points key={layer.id} geometry={visual.geometry} material={visual.material} />;
      })}
    </group>
  );
}
//#endregion WorldPointCloudLayer
//#endregion 🔖️World3dHost
