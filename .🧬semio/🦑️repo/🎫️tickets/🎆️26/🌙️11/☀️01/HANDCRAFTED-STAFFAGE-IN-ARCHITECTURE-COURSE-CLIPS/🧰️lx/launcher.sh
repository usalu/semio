set -u
cd /mnt/c/git/semio || exit 1
bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh" 2>&1 | tail -30
ls -la ~/.local/share/semio/launcher/bin
