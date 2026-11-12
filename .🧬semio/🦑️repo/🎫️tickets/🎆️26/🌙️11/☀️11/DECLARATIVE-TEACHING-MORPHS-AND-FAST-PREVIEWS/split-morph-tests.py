import re, pathlib
M = pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🔀️morph")
old = (M / "🧪️tests" / "🐍️.py").read_text(encoding="utf-8")
head, rest = old.split("class SilhouetteTests", 1)
body, tail = ("class SilhouetteTests" + rest).split("class TokenTests", 1)
tokens_cls, _ = ("class TokenTests" + tail).split('if __name__ == "__main__":', 1)
retext_at = tokens_cls.index("    def test_retext_leaves_only_target")
retext_test = tokens_cls[retext_at:]
tokens_cls = tokens_cls[:retext_at].rstrip() + "\n\n\n"
full_head = head.replace('morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")', 'morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️")\ntokens = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🪙️token.🐍️")').replace("the morph toolkit", "the full morph toolkit")
body = body.replace("morph.token(", "tokens.token(").replace("morph.DATA", "tokens.DATA")
retext_class = "class RetextTests(unittest.TestCase):\n    \"\"\"✏ Formula morphs.\"\"\"\n\n" + retext_test
full = full_head + body + retext_class.rstrip() + '\n\n\nif __name__ == "__main__":\n    unittest.main()\n'
tok_head = head.replace('morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")', 'morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🪙️token.🐍️")').replace("Tests of the morph toolkit: every helper ends with exactly the target in the scene, carries colours and stays fast for 140 sources.", "Tests of the shared morph primitives: tokens, travel and tether.")
tok = tok_head + tokens_cls.rstrip() + '\n\n\nif __name__ == "__main__":\n    unittest.main()\n'
(M / "🧪️tests" / "🎞️full").mkdir(exist_ok=True)
(M / "🧪️tests" / "🪙️token").mkdir(exist_ok=True)
(M / "🧪️tests" / "🎞️full" / "🐍️.py").write_text(full, encoding="utf-8", newline="\n")
(M / "🧪️tests" / "🪙️token" / "🐍️.py").write_text(tok, encoding="utf-8", newline="\n")
