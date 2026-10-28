$ErrorActionPreference = 'Stop'
$workspace = 'C:/git/semio'
$ticket = Join-Path $workspace '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE'
$generated = Join-Path $ticket '🗑️generated/full-physical'
$env:SEMIO_TOOL_PYTHON = Join-Path $workspace '.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:XDG_CACHE_HOME = Join-Path $generated 'cache'
$env:MPLCONFIGDIR = Join-Path $generated 'mpl'
$repo = 'C:/Users/Ueli/AppData/Local/semio/bin/repo.exe'
Set-Location -LiteralPath $workspace
& $repo capacity run --lang=en --memory=16384 --cpu=4 --jobs=4 -- $repo tools exec python --light --lang=en -- -X utf8 '🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py' --language de --audience spectator --video physical_fundamentals --output-root $generated --resolution 640,360 --fps 6 --jobs 4 --log-output --no-play
exit $LASTEXITCODE
