set -u
export PATH="$HOME/.local/share/semio/launcher/bin:$PATH" SEMIO_LANGUAGE=en
cd /mnt/c/git/semio || exit 1
timeout 1500 repo preflight 2>&1 | tail -60
echo "exit=${PIPESTATUS[0]}"
