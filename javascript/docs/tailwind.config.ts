import type { Config } from "tailwindcss";
// import starlightPlugin from '@astrojs/starlight-tailwind';
import { tailwindConfig } from "@semio/core";

const config: Pick<Config, "content" | "presets" | "plugins"> = {
  // content: ["./**/*.{ts,tsx,mdx}"],
  presets: [tailwindConfig],
  // plugins: [starlightPlugin],
};

export default config;