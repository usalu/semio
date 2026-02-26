#!/bin/bash
set -e

# Version and Commit from failed install log logic
COMMIT="c9b91c281ca4919466bd32a6ea2fcdab11102259"
VERSION="1.18.3"
QUALITY="stable"
PLATFORM="linux"
ARCH="x64"

# Verified URL
URL="https://edgedl.me.gvt1.com/edgedl/release2/j0qc3/antigravity/${QUALITY}/${VERSION}-${COMMIT}/${PLATFORM}-${ARCH}/Antigravity-reh.tar.gz"

SERVER_DATA_DIR="/workspace/.antigravity-server-cache"
SERVER_DIR="${SERVER_DATA_DIR}/bin/${VERSION}-${COMMIT}"
SERVER_BIN="${SERVER_DIR}/bin/antigravity-server"

echo "Checking Antigravity Server installation..."

# Remove stale installation lock
rm -f "$SERVER_DATA_DIR/.installation_lock"

if [ ! -f "$SERVER_BIN" ]; then
    echo "Server not found at $SERVER_BIN"
    echo "Downloading Antigravity Server from $URL..."
    
    mkdir -p "$SERVER_DIR"
    
    # Use wget with retry and visible progress
    if ! wget -q --show-progress --tries=3 -O /tmp/server.tar.gz "$URL"; then
        echo "Download failed!"
        exit 1
    fi
    
    echo "Extracting server..."
    tar -xzf /tmp/server.tar.gz -C "$SERVER_DIR" --strip-components=1
    rm /tmp/server.tar.gz
    
    echo "Installation complete."
else
    echo "Server already installed."
fi

# Ensure permissions
chmod +x "$SERVER_BIN"

# We don't need to start it necessarily, the CLI will try to start it.
# But just in case, we can echo the path for verification.
echo "Server binary path: $SERVER_BIN"
