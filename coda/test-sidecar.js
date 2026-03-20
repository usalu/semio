#!/usr/bin/env node

/**
 * Test script to verify sidecar integration without GUI
 * This simulates the main process sidecar spawning logic
 */

import { spawn } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Configuration from main.ts
const SIDECAR_PY_DIR = path.resolve(__dirname, 'engine');
const SIDECAR_CMD = 'uv';
const SIDECAR_BASE_ARGS = ['run', '--active', 'coda.py', '--sidecar'];

console.log(`[TEST] Sidecar directory: ${SIDECAR_PY_DIR}`);
console.log(`[TEST] Sidecar command: ${SIDECAR_CMD} ${SIDECAR_BASE_ARGS.join(' ')}`);

// Check if engine directory exists
if (!fs.existsSync(SIDECAR_PY_DIR)) {
    console.error(`[ERROR] Engine directory not found: ${SIDECAR_PY_DIR}`);
    process.exit(1);
}

// Check if coda.py exists
const codaPyPath = path.join(SIDECAR_PY_DIR, 'coda.py');
if (!fs.existsSync(codaPyPath)) {
    console.error(`[ERROR] coda.py not found: ${codaPyPath}`);
    process.exit(1);
}

console.log('[TEST] Starting sidecar...');

const sidecar = spawn(SIDECAR_CMD, SIDECAR_BASE_ARGS, {
    stdio: ['pipe', 'pipe', 'pipe'],
    cwd: SIDECAR_PY_DIR,
    env: { ...process.env },
});

let readyReceived = false;
let heartbeatReceived = false;

sidecar.stdout.on('data', (data) => {
    const lines = data.toString().trim().split('\n');
    for (const line of lines) {
        if (!line.trim()) continue;
        try {
            const msg = JSON.parse(line);
            console.log(`[SIDECAR] ${JSON.stringify(msg)}`);
            
            // Ready signal
            if (msg.id === null && msg.result?.status === 'ready') {
                readyReceived = true;
                console.log('[TEST] ✅ Ready signal received');
            }
            
            // Heartbeat response
            if (msg.id === null && msg.result?.status === 'alive') {
                heartbeatReceived = true;
                console.log('[TEST] ✅ Heartbeat response received');
            }
        } catch (e) {
            // Ignore unparseable lines
        }
    }
});

sidecar.stderr.on('data', (data) => {
    console.error(`[SIDECAR STDERR] ${data.toString().trim()}`);
});

sidecar.on('error', (err) => {
    console.error(`[ERROR] Sidecar spawn error: ${err.message}`);
    process.exit(1);
});

sidecar.on('exit', (code, signal) => {
    console.log(`[SIDECAR] Exited code=${code} signal=${signal}`);
    
    if (readyReceived) {
        console.log('[TEST] ✅ Integration test PASSED - Ready signal received');
        process.exit(0);
    } else {
        console.log('[TEST] ❌ Integration test FAILED - No ready signal');
        process.exit(1);
    }
});

// Send heartbeat after ready signal
setTimeout(() => {
    if (readyReceived) {
        console.log('[TEST] Sending heartbeat...');
        sidecar.stdin.write(JSON.stringify({ id: 'test-hb', method: 'heartbeat', params: {} }) + '\n');
    }
}, 2000);

// Send test request
setTimeout(() => {
    if (readyReceived) {
        console.log('[TEST] Sending test request...');
        sidecar.stdin.write(JSON.stringify({ id: 'test-req', method: 'get_measures', params: {} }) + '\n');
    }
}, 3000);

// Timeout after 10 seconds
setTimeout(() => {
    console.log('[TEST] Timeout reached');
    sidecar.kill('SIGTERM');
}, 10000);
