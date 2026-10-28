from pathlib import Path
p=Path(__file__).with_name('type-scalar-oracle.py');s=p.read_text(encoding='utf8').replace('X2\\\\539F0020D83EDDED002000730065006D0069006F','X4\\\\0000539F000000200001F9ED0000002000000073000000650000006D000000690000006F');p.write_text(s,encoding='utf8')
