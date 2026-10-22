#!/usr/bin/env python3
"""🧰️ Writes staged/🧩️units/🧪️tests/🌐️languages/🔣️.json: language-agnostic vectors for the .NET, C/C++, WIT and TeX adapters (hand-written expectations). The third-party oracles (dotnet msbuild and build, clang -MM / -E -P / cmake, wit-parser, tectonic when installed) and the stdlib-only python oracle for TeX judge them at test time."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "staged", "🧩️units", "🧪️tests", "🌐️languages", "🔣️.json")

# ------------------------------------------------------------------ .NET
DOTNET_FILES = {
    "global.json": '{"sdk": {"version": "8.0.0", "rollForward": "latestMajor"}}\n',
    "Directory.Build.props": "<Project><PropertyGroup><Deterministic>true</Deterministic></PropertyGroup></Project>\n",
    "lib/lib.csproj": '<Project Sdk="Microsoft.NET.Sdk">\n  <PropertyGroup>\n    <TargetFramework>net8.0</TargetFramework>\n    <Nullable>enable</Nullable>\n  </PropertyGroup>\n</Project>\n',
    "lib/Greeter.cs": "namespace Lab;\n\npublic class Greeter\n{\n    public string Hello(string name) => Decorate(\"hello \" + name);\n\n    private static string Decorate(string text) => text.ToUpperInvariant();\n}\n",
    "lib/obj/ignored.cs": "class Ignored {}\n",
    "lib/bin/ignored.cs": "class IgnoredToo {}\n",
    "app/app.csproj": '<Project Sdk="Microsoft.NET.Sdk">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net8.0</TargetFramework>\n    <AssemblyName>labapp</AssemblyName>\n  </PropertyGroup>\n  <ItemGroup>\n    <ProjectReference Include="..\\lib\\lib.csproj" />\n    <Compile Include="../shared/*.cs" />\n    <Compile Remove="Skipped.cs" />\n  </ItemGroup>\n</Project>\n',
    "app/Program.cs": "using System;\nusing Lab;\n\nConsole.WriteLine(new Greeter().Hello(Shared.Name));\n",
    "app/Skipped.cs": "class Skipped {}\n",
    "shared/Shared.cs": "static class Shared { public static string Name => \"semio\"; }\n",
    "tests/tests.csproj": '<Project Sdk="Microsoft.NET.Sdk">\n  <PropertyGroup>\n    <TargetFramework>net8.0</TargetFramework>\n    <IsPackable>false</IsPackable>\n  </PropertyGroup>\n  <ItemGroup>\n    <PackageReference Include="Microsoft.NET.Test.Sdk" Version="17.8.0" />\n    <PackageReference Include="xunit" Version="2.6.1" />\n    <ProjectReference Include="../lib/lib.csproj" />\n  </ItemGroup>\n</Project>\n',
    "tests/GreeterTests.cs": "public class GreeterTests {}\n",
}
DOTNET = {
    "files": DOTNET_FILES,
    "expect": {
        "dotnet:lib/lib": {"kind": "library", "sources": ["lib/Greeter.cs"], "dependencies": [], "externals": [], "interface": True, "assembly": "lib"},
        "dotnet:app/app": {"kind": "binary", "sources": ["app/Program.cs", "shared/Shared.cs"], "dependencies": [["dotnet:lib/lib", "implementation"]], "externals": [], "interface": False, "assembly": "labapp"},
        "dotnet:tests/tests": {"kind": "test", "sources": ["tests/GreeterTests.cs"], "dependencies": [["dotnet:lib/lib", "implementation"]], "externals": ["nuget:Microsoft.NET.Test.Sdk@17.8.0", "nuget:xunit@2.6.1"], "interface": False, "assembly": "tests"},
    },
    "csproj": [
        {"name": "package-version-child", "text": '<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><PackageReference Include="A"><Version>1.2.3</Version></PackageReference><PackageReference Include="B" Version="2.0.0"/></ItemGroup><PropertyGroup Condition="\'$(X)\'==\'1\'"><AssemblyName>conditional</AssemblyName></PropertyGroup><PropertyGroup><AssemblyName>real</AssemblyName><TargetFrameworks>net8.0;net9.0</TargetFrameworks></PropertyGroup></Project>',
         "expect": {"sdk": "Microsoft.NET.Sdk", "packageReferences": [{"id": "A", "version": "1.2.3"}, {"id": "B", "version": "2.0.0"}], "assemblyName": "real", "frameworks": "net8.0;net9.0", "defaultCompile": True}},
        {"name": "explicit-compile-only", "text": '<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><EnableDefaultCompileItems>false</EnableDefaultCompileItems></PropertyGroup><ItemGroup><Compile Include="../../🎨️palette/🔷️.cs" Link="x.cs" /></ItemGroup><!-- <ProjectReference Include="commented.csproj" /> --></Project>',
         "expect": {"sdk": "Microsoft.NET.Sdk", "packageReferences": [], "assemblyName": None, "frameworks": None, "defaultCompile": False, "compileIncludes": ["../../🎨️palette/🔷️.cs"], "projectReferences": []}},
    ],
    "globs": [
        {"glob": "**/*.cs", "match": ["a.cs", "x/b.cs", "x/y/c.cs"], "reject": ["a.txt", "x/y/c.csproj"]},
        {"glob": "src/*.cs", "match": ["src/a.cs"], "reject": ["src/x/a.cs", "a.cs"]},
        {"glob": "a?.cs", "match": ["ab.cs"], "reject": ["abc.cs"]},
        {"glob": "🎨️palette/**", "match": ["🎨️palette/x/y.cs"], "reject": ["palette/x.cs"]},
    ],
}

# ------------------------------------------------------------------ C/C++
CPP_PAIRS = [
    {"name": "comment-edit", "same": True, "before": "int f(int x);\n// old\n", "after": "int f(int x);\n/* new\n   comment */\n"},
    {"name": "whitespace-edit", "same": True, "before": "int   f( int x );\n#define A   1\n", "after": "int f(int x);\n#define A 1\n"},
    {"name": "trailing-comment", "same": True, "before": "#define LIMIT 10\n", "after": "#define LIMIT 10 // ten\n"},
    {"name": "parameter-type", "same": False, "before": "int f(int x);\n", "after": "int f(long x);\n"},
    {"name": "macro-value", "same": False, "before": "#define LIMIT 10\n", "after": "#define LIMIT 11\n"},
    {"name": "new-declaration", "same": False, "before": "int f(int x);\n", "after": "int f(int x);\nint g(void);\n"},
    {"name": "slashes-in-string", "same": False, "before": 'const char *u = "http://a";\n', "after": 'const char *u = "http://b";\n'},
    {"name": "inline-body-edit", "same": False, "before": "static inline int one(void) { return 1; }\n", "after": "static inline int one(void) { return 2; }\n"},
]
CPP_INCLUDES = [
    {"name": "quoted-angle-comments", "source": '#include "a.h"\n#include <b/c.h>\n// #include "gone.h"\n/* #include "gone2.h" */\n  #  include "d.h"\n', "expect": [["a.h", False], ["b/c.h", True], ["d.h", False]]},
]
CPP_WORKSPACE = {
    "CMakeLists.txt": "cmake_minimum_required(VERSION 3.16)\nproject(lab CXX)\nset(CMAKE_CXX_STANDARD 17)\nadd_library(foo STATIC foo/src/foo.cpp)\ntarget_include_directories(foo PUBLIC foo/include PRIVATE foo/src)\nadd_library(bar STATIC bar/bar.cpp)\ntarget_include_directories(bar PUBLIC bar/include)\ntarget_link_libraries(bar PUBLIC foo)\nadd_executable(app app/main.cpp)\ntarget_link_libraries(app PRIVATE bar)\n",
    "foo/include/foo/api.h": "#pragma once\n// public API of foo\nint foo_value(int x);\n",
    "foo/src/foo.cpp": '#include "foo/api.h"\n#include "internal.h"\n\nint foo_value(int x) { return internal_scale(x) + 1; }\n',
    "foo/src/internal.h": "#pragma once\ninline int internal_scale(int x) { return x * 2; }\n",
    "bar/include/bar/api.h": '#pragma once\n#include "foo/api.h"\nint bar_value(int x);\n',
    "bar/bar.cpp": '#include "bar/api.h"\n\nint bar_value(int x) { return foo_value(x) + 10; }\n',
    "app/main.cpp": '#include <cstdio>\n#include "bar/api.h"\n\nint main() { std::printf("%d\\n", bar_value(1)); return 0; }\n',
}
CPP = {
    "pairs": CPP_PAIRS, "includes": CPP_INCLUDES, "workspace": CPP_WORKSPACE,
    "expect": {
        "cpp:foo": {"kind": "library", "dependencies": [], "sources": ["foo/include/foo/api.h", "foo/src/foo.cpp"], "inputs": ["CMakeLists.txt", "foo/src/internal.h"]},
        "cpp:bar": {"kind": "library", "dependencies": [["cpp:foo", "interface"]], "sources": ["bar/bar.cpp", "bar/include/bar/api.h"], "inputs": ["CMakeLists.txt"]},
        "cpp:app": {"kind": "binary", "dependencies": [["cpp:bar", "both"], ["cpp:foo", "both"]], "sources": ["app/main.cpp"], "inputs": ["CMakeLists.txt", "bar/bar.cpp", "foo/src/foo.cpp", "foo/src/internal.h"]},
    },
}

# ------------------------------------------------------------------ WIT
BASE = "package example:calc@0.1.0;\n\ninterface types {\n  record point { x: u32, y: u32 }\n  variant shape { dot(point), line(tuple<point, point>) }\n  type id = u64;\n  area: func(s: shape) -> u64;\n}\n\nworld calc {\n  import types;\n  export run: func() -> string;\n}\n"
WIT_PAIRS = [
    {"name": "comments-and-docs", "same": True, "before": BASE, "after": "// header\npackage example:calc@0.1.0;\n\n/// the types\ninterface types {\n  /* point */ record point { x: u32, y: u32 }\n  variant shape { dot(point), line(tuple<point, point>) }\n  type id = u64;\n  area: func(s: shape) -> u64;\n}\n\nworld calc {\n  import types;\n  export run: func() -> string;\n}\n"},
    {"name": "formatting", "same": True, "before": BASE, "after": BASE.replace("record point { x: u32, y: u32 }", "record point {\n    x: u32,\n    y: u32\n  }").replace("\n\n", "\n")},
    {"name": "item-order", "same": True, "reorder": True, "before": BASE, "after": "package example:calc@0.1.0;\n\nworld calc {\n  export run: func() -> string;\n  import types;\n}\n\ninterface types {\n  area: func(s: shape) -> u64;\n  type id = u64;\n  variant shape { dot(point), line(tuple<point, point>) }\n  record point { x: u32, y: u32 }\n}\n"},
    {"name": "field-type", "same": False, "before": BASE, "after": BASE.replace("x: u32", "x: u64")},
    {"name": "field-order", "same": False, "before": BASE, "after": BASE.replace("x: u32, y: u32", "y: u32, x: u32")},
    {"name": "new-function", "same": False, "before": BASE, "after": BASE.replace("area: func", "perimeter: func(s: shape) -> u64;\n  area: func")},
    {"name": "export-name", "same": False, "before": BASE, "after": BASE.replace("export run", "export go")},
    {"name": "version", "same": False, "before": BASE, "after": BASE.replace("@0.1.0", "@0.2.0")},
]
WIT_INVALID = [
    {"name": "unbalanced", "files": {"a.wit": "package a:b@0.1.0;\ninterface i {\n  f: func();\n"}},
    {"name": "unterminated-comment", "files": {"a.wit": "package a:b@0.1.0;\n/* open\ninterface i {}\n"}},
]
WIT_WORKSPACE = {
    "wit/world.wit": "package example:app@0.1.0;\n\nworld app {\n  use example:types/shapes@0.1.0.{point};\n  export origin: func() -> point;\n}\n",
    "wit/deps/types/types.wit": "package example:types@0.1.0;\n\ninterface shapes {\n  record point { x: u32, y: u32 }\n  type id = u64;\n}\n",
    "other/other.wit": "package example:other@1.0.0;\n\ninterface x { f: func(); }\n",
}
WIT = {
    "pairs": WIT_PAIRS, "invalid": WIT_INVALID, "workspace": WIT_WORKSPACE,
    "expect": {
        "wit:example:app@0.1.0": {"sources": ["wit/world.wit"], "dependencies": ["wit:example:types@0.1.0"]},
        "wit:example:types@0.1.0": {"sources": ["wit/deps/types/types.wit"], "dependencies": []},
        "wit:example:other@1.0.0": {"sources": ["other/other.wit"], "dependencies": []},
    },
    "checks": [
        {"name": "unknown-import-name", "files": {"wit/world.wit": WIT_WORKSPACE["wit/world.wit"].replace("{point}", "{pointy}")}, "diagnostic": "example:types/shapes defines no pointy"},
        {"name": "unknown-interface", "files": {"wit/world.wit": WIT_WORKSPACE["wit/world.wit"].replace("shapes", "shapez")}, "diagnostic": "example:types has no interface or world shapez"},
        {"name": "fine", "files": {}, "diagnostic": None},
    ],
}

# ------------------------------------------------------------------ TeX
AUX = "\\relax\n\\providecommand\\hyper@newdestlabel[2]{}\n\\citation{knuth}\n\\bibstyle{plain}\n\\bibdata{refs}\n\\bibcite{knuth}{1}\n\\bibcite{abel}{2}\n\\@writefile{toc}{\\contentsline {section}{\\numberline {1}Intro}{1}{section.1}}\n\\newlabel{sec:b}{{2}{3}{Next}{section.2}{}}\n\\newlabel{sec:a}{{1}{1}{Intro}{section.1}{}}\n\\gdef\\@abspage@last{4}\n"
TEX = {
    "aux": AUX,
    "aux-expect": "\\newlabel{sec:a}{{1}{1}{Intro}{section.1}{}}\n\\newlabel{sec:b}{{2}{3}{Next}{section.2}{}}\n\\bibcite{abel}{2}\n\\bibcite{knuth}{1}\n\\bibstyle{plain}\n\\bibdata{refs}\n\\@writefile{toc}{\\contentsline {section}{\\numberline {1}Intro}{1}{section.1}}",
    "aux-pairs": [
        {"name": "label-order", "same": True, "before": AUX, "after": AUX.replace("\\newlabel{sec:b}{{2}{3}{Next}{section.2}{}}\n\\newlabel{sec:a}{{1}{1}{Intro}{section.1}{}}\n", "\\newlabel{sec:a}{{1}{1}{Intro}{section.1}{}}\n\\newlabel{sec:b}{{2}{3}{Next}{section.2}{}}\n")},
        {"name": "bookkeeping", "same": True, "before": AUX, "after": AUX.replace("\\gdef\\@abspage@last{4}", "\\gdef\\@abspage@last{9}").replace("\\citation{knuth}\n", "")},
        {"name": "label-number", "same": False, "before": AUX, "after": AUX.replace("{{2}{3}{Next}", "{{3}{3}{Next}")},
        {"name": "new-label", "same": False, "before": AUX, "after": AUX + "\\newlabel{fig:x}{{1}{2}{Fig}{figure.1}{}}\n"},
        {"name": "toc-order", "same": False, "before": AUX + "\\@writefile{toc}{\\contentsline {section}{\\numberline {2}Next}{3}{section.2}}\n", "after": AUX.replace("\\@writefile{toc}{\\contentsline {section}{\\numberline {1}Intro}{1}{section.1}}\n", "\\@writefile{toc}{\\contentsline {section}{\\numberline {2}Next}{3}{section.2}}\n") + "\\@writefile{toc}{\\contentsline {section}{\\numberline {1}Intro}{1}{section.1}}\n"},
    ],
    "scans": [
        {"name": "references", "source": "\\documentclass[a4paper]{report}\n\\usepackage{amsmath,local}\n% \\input{commented}\n\\externaldocument[b-]{other/b}\n\\begin{document}\n\\input{chapters/one}\n\\include{chapters/two}\n\\includegraphics[width=3cm]{fig/logo}\n\\bibliography{refs,more}\n50\\%\\input{after-percent}\n\\end{document}\n",
         "expect": {"inputs": ["chapters/one", "chapters/two", "after-percent"], "bibliographies": ["refs", "more"], "graphics": ["fig/logo"], "packages": ["amsmath", "local"], "classes": ["report"], "externalDocuments": ["other/b"], "isDocument": True}},
        {"name": "fragment", "source": "\\section{x}\n\\label{a}\n", "expect": {"inputs": [], "bibliographies": [], "graphics": [], "packages": [], "classes": [], "externalDocuments": [], "isDocument": False}},
    ],
    "workspace": {
        "report/main.tex": "\\documentclass{article}\n\\usepackage{style}\n\\externaldocument{appendix}\n\\begin{document}\n\\input{parts/one}\n\\includegraphics{pic}\n\\bibliography{refs}\n\\end{document}\n",
        "report/appendix.tex": "\\documentclass{article}\n\\begin{document}\n\\section{A}\\label{sec:app}\n\\end{document}\n",
        "report/parts/one.tex": "\\section{One}\n\\input{parts/two}\n",
        "report/parts/two.tex": "text\n",
        "report/pic.png": "png", "report/refs.bib": "@book{k,title={t}}\n", "report/style.sty": "\\ProvidesPackage{style}\n",
        "notes/fragment.tex": "\\section{only a fragment}\n",
    },
    "expect": {
        "tex:report/main": {"sources": ["report/main.tex"], "inputs": ["report/parts/one.tex", "report/parts/two.tex", "report/pic.png", "report/refs.bib", "report/style.sty"], "dependencies": ["tex:report/appendix"]},
        "tex:report/appendix": {"sources": ["report/appendix.tex"], "inputs": [], "dependencies": []},
    },
}

fixture = {"dotnet": DOTNET, "cpp": CPP, "wit": WIT, "tex": TEX}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("wrote", OUT)
