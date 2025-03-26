import type { Config } from "tailwindcss";
// import starlightPlugin from '@astrojs/starlight-tailwind';
import { tailwindConfig } from "@semio/js";

const config: Pick<Config, "presets"> = { // | "content" | "plugins" 
  // content: ["./**/*.{ts,tsx,mdx}"],
  presets: [tailwindConfig],
  // plugins: [starlightPlugin],
};

export default config;