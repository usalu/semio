import type { Config } from "tailwindcss";
import { tailwindConfig } from "@semio/core";

const config: Pick<Config, "content" | "presets"> = {
  // content: ["*.{ts,tsx,mdx}"],
  presets: [tailwindConfig],
};

export default config;