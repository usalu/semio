import hashlib
import marshal
import sys
import types
import unittest


def digest(code):
    def normalize(value):
        if isinstance(value, types.CodeType):
            return value.replace(co_filename="<module>", co_consts=tuple(normalize(x) for x in value.co_consts))
        return value
    return hashlib.sha256(marshal.dumps((sys.version_info[:3], normalize(code)))).hexdigest()


def load(source, filename):
    scope = {"capture": lambda: sys._getframe(1).f_code}
    exec(compile(source, filename, "exec"), scope)
    return scope


class ModuleIdentityTest(unittest.TestCase):
    def test_global_constant_replacement(self):
        first = load("module_code = capture()\nVALUE = 3\ndef geometry(): return VALUE\n", "first.py")
        second = load("module_code = capture()\nVALUE = 4\ndef geometry(): return VALUE\n", "second.py")
        self.assertEqual(first["geometry"].__code__.co_code, second["geometry"].__code__.co_code)
        self.assertNotEqual(digest(first["module_code"]), digest(second["module_code"]))
        same = load("module_code = capture()\nVALUE = 3\ndef geometry(): return VALUE\n", "another.py")
        self.assertEqual(digest(first["module_code"]), digest(same["module_code"]))
        first["VALUE"] = 9
        self.assertEqual(digest(first["module_code"]), digest(same["module_code"]))
        self.assertNotEqual(first["geometry"](), same["geometry"]())


if __name__ == "__main__":
    unittest.main()
