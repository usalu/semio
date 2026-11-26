"""🧪️ Independently check authored browser font custody with Python's graphlib."""
from graphlib import TopologicalSorter
import json
from pathlib import Path
import sys

def main():
    fixture_path=Path(sys.argv[1])
    fixture=json.loads(fixture_path.read_bytes())
    root=fixture_path.resolve().parents[7]
    for row in fixture["projects"]:
        project=json.loads((root/row["path"]).read_bytes())
        recipe=project["targets"][row["target"]]["metadata"]["semio"]["release"]
        graph={job["id"]:[edge.removeprefix("job:") for edge in job["after"]] for job in recipe["jobs"]}
        ordered=list(TopologicalSorter(graph).static_order())
        sequence=[name for name in ordered if name in fixture["browserDependencyOrder"]]
        browser=[edge.removeprefix("job:") for edge in recipe["stageAfter"]["browser"]]
        stage={edge.removeprefix("job:") for edges in recipe["stageAfter"].values() for edge in edges}
        pending=[job["id"] for job in recipe["jobs"] if "lane" in job or job["id"] not in stage]
        prerequisites=set()
        while pending:
            job=pending.pop()
            if job not in prerequisites:
                prerequisites.add(job)
                pending.extend(graph[job])
        cargo=[name for name in ordered if name in prerequisites]
        assert sequence==browser==fixture["browserDependencyOrder"],(row["path"],sequence,browser)
        assert cargo==row["cargoPrerequisites"],(row["path"],cargo)
        print(json.dumps(dict(path=row["path"],sequence=sequence,browser=browser,prerequisites=cargo),ensure_ascii=False,separators=(",",":")))

if __name__=="__main__":
    main()
