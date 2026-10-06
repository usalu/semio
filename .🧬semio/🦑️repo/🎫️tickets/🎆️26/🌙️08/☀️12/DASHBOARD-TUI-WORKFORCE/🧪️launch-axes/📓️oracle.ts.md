
test("Ajv and Nx independently admit every explicit renderer language selection", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🗣️launch-axes/🔣️.json",import.meta.url),"utf8"));
  const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🗣️launch-axes/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
  const {createTaskGraph}=await import("nx/src/tasks-runner/create-task-graph.js");
  const targets=Object.fromEntries(fixture.renderers.map((row:{target:string})=>[row.target,{executor:"nx:run-commands",options:{command:"bun fixture"}}]));
  const graph={nodes:{"@semio-tech/framework-os-dev":{name:"@semio-tech/framework-os-dev",type:"lib",data:{root:"dev",targets}}},dependencies:{"@semio-tech/framework-os-dev":[]}};
  for(const row of fixture.renderers)for(const locale of fixture.locales)for(const terminology of fixture.terminologies){
    const expected={SEMIO_LOCKED_LOCALE:locale,SEMIO_LOCKED_TERMINOLOGY:terminology};
    const tasks=createTaskGraph(graph,{},["@semio-tech/framework-os-dev"],[row.target],undefined,expected).tasks;
    expect(Object.keys(tasks)).toEqual([`@semio-tech/framework-os-dev:${row.target}`]);
    expect(tasks[Object.keys(tasks)[0]!].overrides).toEqual(expected);
  }
});
