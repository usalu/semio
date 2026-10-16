// 🔐️ Puzzle lock entries: the scene owner joins the plugin lock; 2d and 5d gain the scene and retained path dependencies.
import {lockEnsure} from "./dd.ts";
const Z = "✏️s/🔌️plugins/🧩️puzzle";
const SCENE = "semio-s-plugin-puzzle-board-scene";
lockEnsure(`${Z}/Cargo.lock`, SCENE, "0.1.0", ["semio-framework-pack-json", "semio-framework-value", "serde_json"], []);
for (const [art, pkg] of [["◻️2d", "semio-s-artifact-puzzle-2d"], ["🖐️5d", "semio-s-artifact-puzzle-5d"]] as const) {
  const lock = `${Z}/🗿️artifacts/${art}/Cargo.lock`;
  lockEnsure(lock, SCENE, "0.1.0", ["semio-framework-pack-json", "semio-framework-value", "serde_json"], [pkg]);
  lockEnsure(lock, "semio-s-plugin-puzzle-jobs", "0.1.0", [], [pkg]);
  lockEnsure(lock, "semio-s-plugin-puzzle-retained", "0.1.0", [], [pkg]);
}
