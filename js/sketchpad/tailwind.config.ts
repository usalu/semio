import { tailwindConfig } from "@semio/js";
import type { Config } from "tailwindcss";

const config: Pick<Config, "content" | "presets"> = {
  content: ["./**/*.{ts,tsx,mdx}"],
  presets: [tailwindConfig],
};

export default config;
