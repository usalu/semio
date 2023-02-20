conda create -n semio python=3.11
conda activate semio
pip install -r src\packages\semio\requirements.txt
pip install -r src\backend\gateway\server\requirements.txt
pip install -r src\backend\assembler\requirements.txt
