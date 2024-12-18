if (-not (Test-Path ".venv")) {
    poetry install
}
.venv/Scripts/activate.ps1
if (Test-Path "build") {
    Remove-Item "build" -Recurse
}
if (Test-Path "dist") {
    Remove-Item "dist" -Recurse
}
pyinstaller --name "semio-engine" --windowed --clean --noconfirm --add-data "../../icons/semio_16x16.png;icons/" --add-data "../../icons/semio_32x32.png;icons/" --add-data "../../icons/semio_48x48.png;icons/" --add-data "../../icons/semio_128x128.png;icons/" --add-data "../../icons/semio_256x256.png;icons/" --icon "..\..\icons\semio.ico" engine.py
.\post-build.ps1