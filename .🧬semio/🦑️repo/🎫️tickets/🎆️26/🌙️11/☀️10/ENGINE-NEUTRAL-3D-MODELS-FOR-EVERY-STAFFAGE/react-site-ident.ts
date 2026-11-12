import { ENGLISH } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
const keys = Object.keys(ENGLISH).filter((key) => /^(model\.|viewpoints\.(name|phrase|missing|announce|helpSpatial)\.?(axonometric|model|Spatial)?|detail\.format\.(glb|fbx|model))/u.test(key));
console.log("[DEBUG]", keys.length, keys.filter((k) => (ENGLISH as Record<string, string>)[k] === (GERMAN as Record<string, string>)[k]));
