import { fileURLToPath } from "node:url";
import config from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

export default {
  ...config,
  test: {
    ...config.test,
    setupFiles: [...config.test.setupFiles, fileURLToPath(new URL("./react-native-browser.setup.mjs", import.meta.url))],
  },
};
