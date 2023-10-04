The build.ps1 script will run and probably produce a binary that might run on your machine but not generally!

After poetry install you need to add the pyinstaller hooks (if path variables are added by packages) by creating the following file:

.venv\Lib\site-packages\PyInstaller\hooks\hook-NAME.py

from PyInstaller.utils.hooks import collect_data_files

datas = collect_data_files ( 'NAME' )

where NAME is the module name which adds hooks.

Current hooks are:

.venv\Lib\site-packages\PyInstaller\hooks\hook-grpc.py

from PyInstaller.utils.hooks import collect_data_files

datas = collect_data_files ( 'grpc' )
