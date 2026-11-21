import config from "./vite-play.config.ts";

export default { ...config, cacheDir: process.env.SEMIO_VITE_CACHE_DIRECTORY ?? config.cacheDir };
