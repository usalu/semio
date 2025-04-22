/// <reference types="vite/client" />

declare module '*.wasm?url' {
    const value: string;
    export default value;
} 