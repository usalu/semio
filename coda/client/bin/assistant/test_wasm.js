const fs = require('fs');
const path = require('path');

// Read the wasm module
async function test() {
    try {
        const pkg = require('./sketchpad-rs/pkg/sketchpad_rs.js');
        // Node requires we load the wasm bytes differently if not using a bundler.
        // Actually, we can just run a python server and use playwright or we can just try to run cargo test in sketchpad-rs.
    } catch(e) {
        console.error(e);
    }
}
test();
