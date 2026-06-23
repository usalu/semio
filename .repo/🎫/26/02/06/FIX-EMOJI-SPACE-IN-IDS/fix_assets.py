import os
import re


def fix_paths():
    root_dir = "/workspaces/semio"

    # 1. Fix assets paths
    # Inside compose/, ../../assets -> ../assets
    # (Matches what we did before)

    # 2. Fix sql/sqlite paths
    # compose/rs/../../sql/sqlite/compose/schema.sql -> compose/rs/../sqlite/schema.sql
    # compose/go/../../sql/sqlite/compose/schema.sql -> compose/go/../sqlite/schema.sql

    for subdir in ["compose", "repo", "coda"]:
        base_path = os.path.join(root_dir, subdir)
        if not os.path.exists(base_path):
            continue

        for root, dirs, files in os.walk(base_path):
            if "node_modules" in root:
                continue
            for file in files:
                if file.endswith(
                    (
                        ".ts",
                        ".tsx",
                        ".json",
                        ".go",
                        ".py",
                        ".css",
                        ".mdx",
                        ".md",
                        ".rs",
                        ".cs",
                        ".d",
                    )
                ):
                    file_path = os.path.join(root, file)
                    try:
                        with open(file_path, "r", encoding="utf-8") as f:
                            content = f.read()

                        new_content = content

                        # Fix assets patterns
                        if subdir == "compose":
                            # Replace (../)^n assets with (../)^(n-1) assets
                            # pattern = re.compile(r'(\.\./){2,}assets')
                            def sub_assets(m):
                                count = m.group(0).count("../")
                                return "../" * (count - 1) + "assets"

                            new_content = re.sub(
                                r"(\.\./){2,}assets", sub_assets, new_content
                            )

                        # Fix sql patterns for compose/
                        if subdir == "compose":
                            # ../../sql/sqlite/compose/ -> ../sqlite/
                            def sub_sql_compose(m):
                                count = m.group(0).count("../")
                                # if count is 2 (../../sql/sqlite/compose/), we want ../sqlite/
                                # so count - 1
                                return "../" * (count - 1) + "sqlite/"

                            new_content = re.sub(
                                r"(\.\./)+sql/sqlite/compose/", sub_sql_compose, new_content
                            )

                            # Handle cases without trailing slash
                            new_content = re.sub(
                                r"(\.\./)+sql/sqlite/compose",
                                lambda m: (
                                    "../" * (m.group(0).count("../") - 1) + "sqlite"
                                ),
                                new_content,
                            )

                        # Fix sql patterns for repo/
                        if subdir == "repo":
                            # ../../sql/sqlite/repo/ -> ../sqlite/
                            def sub_sql_repo(m):
                                count = m.group(0).count("../")
                                return "../" * (count - 1) + "sqlite/"

                            new_content = re.sub(
                                r"(\.\./)+sql/sqlite/repo/", sub_sql_repo, new_content
                            )
                            new_content = re.sub(
                                r"(\.\./)+sql/sqlite/repo",
                                lambda m: (
                                    "../" * (m.group(0).count("../") - 1) + "sqlite"
                                ),
                                new_content,
                            )

                        if new_content != content:
                            print(f"Fixing paths in {file_path}")
                            with open(file_path, "w", encoding="utf-8") as f:
                                f.write(new_content)

                    except Exception as e:
                        print(f"Error processing {file_path}: {e}")

    # Also fix AGENTS.md at root
    agents_path = os.path.join(root_dir, "AGENTS.md")
    if os.path.exists(agents_path):
        with open(agents_path, "r", encoding="utf-8") as f:
            content = f.read()
        new_content = content.replace("sql/sqlite/repo/", "repo/sqlite/")
        new_content = new_content.replace("sql/sqlite/compose/", "compose/sqlite/")
        new_content = new_content.replace("./sql/sqlite/", "./compose/sqlite/")
        if new_content != content:
            print(f"Fixing paths in {agents_path}")
            with open(agents_path, "w", encoding="utf-8") as f:
                f.write(new_content)


if __name__ == "__main__":
    fix_paths()
