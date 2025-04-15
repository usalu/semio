import { defineConfig } from 'vite';
import { builtinModules } from 'module';
import path from 'path';

export default defineConfig({
    resolve: {
        alias: {
            "@semio/js": path.resolve(__dirname, "../core"),
            "@semio/assets": path.resolve(__dirname, "../../assets")
        }
    },
    build: {
        outDir: '.vite/build',
        lib: {
            entry: 'preload.ts',
            formats: ['cjs'],
            fileName: () => 'preload.js',
        },
        rollupOptions: {
            external: [
                'electron',
                ...builtinModules,
            ],
        },
        emptyOutDir: false,
        sourcemap: 'inline',
    },
});
