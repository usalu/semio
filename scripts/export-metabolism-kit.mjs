#!/usr/bin/env node

// This script is intended to be run via npm/vitest with TypeScript support
// Use: npx vitest run --no-coverage -t "export metabolism" 

import { writeFile } from 'fs/promises';
import { join } from 'path';

// When run in a test context with proper TypeScript support
export async function exportMetabolismToAssets() {
  try {
    const { MetabolismKit } = await import('../assets/index.ts');
    const { exportKit } = await import('../js/js/semio.ts');

    const kit = MetabolismKit;
    const files = new Map();

    const zipBlob = await exportKit(kit, files);

    const buffer = Buffer.from(await zipBlob.arrayBuffer());

    const outputPath = join(process.cwd(), 'assets', 'metabolism.zip');
    await writeFile(outputPath, buffer);

    console.log(`Exported kit to ${outputPath}`);
    console.log(`Kit: ${kit.name} v${kit.version}`);
    console.log(`Types: ${kit.types?.length || 0}`);
    console.log(`Designs: ${kit.designs?.length || 0}`);
    console.log(`Interfaces: ${kit.interfaces?.length || 0}`);
    console.log(`Qualities: ${kit.qualities?.length || 0}`);
    console.log(`Files: ${kit.files?.length || 0}`);
    console.log(`Size: ${(buffer.length / 1024).toFixed(2)} KB`);
    
    return outputPath;
  } catch (error) {
    console.error('Failed to export metabolism kit:', error);
    throw error;
  }
}

// If run directly (not imported), execute
if (import.meta.url === `file://${process.argv[1]}`) {
  console.error('This script requires TypeScript support.');
  console.error('Run via: npx tsx scripts/export-metabolism-kit.mjs');
  console.error('Or add a test case that calls exportMetabolismToAssets()');
  process.exit(1);
}
