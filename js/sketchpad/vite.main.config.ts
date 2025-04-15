import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
    resolve: {
        alias: {
            "@semio/js": path.resolve(__dirname, "../core"),
            "@semio/assets": path.resolve(__dirname, "../../assets")
        }
    }
});
