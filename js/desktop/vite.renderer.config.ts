import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig(async () => {
  
  const tailwind = await import("@tailwindcss/vite");
  return {
    plugins: [tailwind.default(), react()],
  };
});
