import { mergeConfig } from "vitest/config";
import base from "../../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🎚️config/🟦️.ts";
import { storeAliases } from "./emulate-root-declarations.ts";

export default mergeConfig(base, {
  resolve: {
    alias: storeAliases({ yaml: "2.9.0", "@tailwindcss/vite": "4.3.3", "@vitejs/plugin-react": "5.2.0", i18next: "25.10.10", "react-i18next": "16.6.6", immer: "10.1.1", "@xyflow/react": "12.12.0", "@react-three/fiber": "9.8.2" }),
  },
});
