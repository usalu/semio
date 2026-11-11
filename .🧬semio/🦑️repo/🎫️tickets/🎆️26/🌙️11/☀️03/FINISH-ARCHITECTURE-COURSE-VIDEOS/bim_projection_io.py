"""💾 Preserve the current projection inputs independently of optional metadata."""
import json,os,pickle
from pathlib import Path
def checkpoint(folder,facets,matrix,native):
    """📌 Atomically preserve the exact current call before projection begins."""
    folder=Path(folder);essential={"facets":native(facets),"matrix":native(matrix)}
    temporary=folder/"bim-projection-current.tmp"
    with temporary.open("wb") as stream:pickle.dump(essential,stream,protocol=pickle.HIGHEST_PROTOCOL)
    os.replace(temporary,folder/"bim-projection-current.pkl")
    return essential
def write(folder,facets,matrix,metadata,codec,native):
    """💾 Preserve essential values before attempting optional portable metadata."""
    folder=Path(folder);essential=checkpoint(folder,facets,matrix,native)
    try:
        record={"facets":codec.encode(essential["facets"]),"matrix":codec.encode(essential["matrix"]),"item":codec.encode(native(metadata))}
        (folder/"bim-projection-failure.json").write_text(json.dumps(record,ensure_ascii=False),encoding="utf-8")
    except Exception as error:
        print("[DEBUG] Optional projection JSON unavailable:",type(error).__name__,str(error),flush=True)
    return essential
