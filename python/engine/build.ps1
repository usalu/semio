.venv/Scripts/activate.ps1
# copy-item "hooks\*" ".venv\Lib\site-packages\PyInstaller\hooks"
pyinstaller --windowed --clean --noconfirm --name "semio" --add-data "../../icons/semio_16x16.png;icons/" --add-data "../../icons/semio_32x32.png;icons/" --add-data "../../icons/semio_48x48.png;icons/" --add-data "../../icons/semio_128x128.png;icons/" --add-data "../../icons/semio_256x256.png;icons/" --icon "..\..\icons\semio.ico" --hidden-import platformdirs server.py
copy-item "Scripts\*" "dist\semio"
