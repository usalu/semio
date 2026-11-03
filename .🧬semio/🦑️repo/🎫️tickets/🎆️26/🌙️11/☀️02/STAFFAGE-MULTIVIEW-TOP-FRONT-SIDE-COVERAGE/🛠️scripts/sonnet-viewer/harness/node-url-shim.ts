export const fileURLToPath = (url: string | URL): string => String(url);
export const pathToFileURL = (path: string): URL => new URL(`file://${path}`);
export default { fileURLToPath, pathToFileURL };
