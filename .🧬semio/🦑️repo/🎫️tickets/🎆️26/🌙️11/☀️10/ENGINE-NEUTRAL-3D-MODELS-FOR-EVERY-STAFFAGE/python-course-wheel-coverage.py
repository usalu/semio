import sys, tomllib
from packaging import tags
from packaging.utils import parse_wheel_filename

def platforms(kind):
    versions = [(10, minor) for minor in range(16, 3, -1)] + [(major, 0) for major in range(26, 10, -1)]
    if kind == "macos-x64": return [f"macosx_{major}_{minor}_{arch}" for major, minor in versions for arch in ("x86_64", "universal2", "intel", "fat64", "fat32")]
    if kind == "macos-arm64": return [f"macosx_{major}_{minor}_{arch}" for major, minor in versions if major >= 11 for arch in ("arm64", "universal2")]
    if kind == "linux-x64": return [f"manylinux_2_{minor}_x86_64" for minor in range(39, 16, -1)] + ["manylinux2014_x86_64", "manylinux2010_x86_64", "manylinux1_x86_64", "linux_x86_64"]
    if kind == "linux-arm64": return [f"manylinux_2_{minor}_aarch64" for minor in range(39, 16, -1)] + ["manylinux2014_aarch64", "linux_aarch64"]
    if kind == "windows-x64": return ["win_amd64"]
    if kind == "windows-arm64": return ["win_arm64"]

lock = tomllib.load(open(sys.argv[1], "rb"))
for kind in ("macos-arm64", "macos-x64", "linux-x64", "linux-arm64", "windows-x64", "windows-arm64"):
    accepted = {str(tag) for tag in (*tags.cpython_tags((3, 12), platforms=platforms(kind)), *tags.compatible_tags((3, 12), platforms=platforms(kind)))}
    missing = []
    for package in lock["package"]:
        wheels = package.get("wheels", [])
        names = [wheel["url"].rsplit("/", 1)[1] for wheel in wheels]
        fits = any(any(str(tag) in accepted for tag in parse_wheel_filename(name)[3]) for name in names)
        if not fits and (wheels or "sdist" in package):
            missing.append(f"{package['name']}{'(sdist)' if 'sdist' in package else '(none)'}")
    print(kind, missing)
