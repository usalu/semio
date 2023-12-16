using System;
using System.Drawing;
using System.Linq;
using Grasshopper.Kernel;
using Semio.Grasshopper.Goos;
using Semio.Grasshopper.Params;

namespace Semio.Grasshopper.Components;

public class ApiComponent : SemioComponent
{
    public ApiComponent()
        : base("API", "API",
            "Construct, deconstruct or modify an API.",
            "semio2", "Models")
    {
    }


    protected override Bitmap Icon =>
        //You can add image files to your project resources and access them like this:
        // return Resources.IconForThisComponent;
        null;


    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ApiParam(), "API", "A?", "An API to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Endpoint", "E", "The endpoint of the API.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Token", "T?", "The optional token of the API.", GH_ParamAccess.item);
        pManager[0].Optional = true;
    }


    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ApiParam(), "API", "A", "The constructed or modified API.", GH_ParamAccess.item);
        pManager.AddTextParameter("Endpoint", "E", "The endpoint of the API.", GH_ParamAccess.item);
        pManager.AddTextParameter("Token", "T?", "The optional token of the API.", GH_ParamAccess.item);
    }


    protected override void SolveInstance(IGH_DataAccess DA)
    {
        ApiGoo apiGoo = new();
        var endpoint = "";
        var token = "";

        DA.GetData(0, ref apiGoo);
        if (DA.GetData(1, ref endpoint))
            apiGoo.Value.Endpoint = endpoint;
        if (DA.GetData(2, ref token))
            apiGoo.Value.Token = token;

        DA.SetData(0, apiGoo);
        DA.SetData(1, endpoint);
        DA.SetData(2, token);
    }
}
public class KitComponent : SemioComponent
{
    public KitComponent()
        : base("Kit", "K",
            "Construct, deconstruct or modify a kit.",
            "semio2", "Models")
    {
    }


    protected override Bitmap Icon =>
        //You can add image files to your project resources and access them like this:
        // return Resources.IconForThisComponent;
        null;


    public override Guid ComponentGuid => new("7EBFD56F-3B62-4676-AF01-24EC8D85231D");

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K?", "A kit to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Name", "N", "The name of the kit.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Description", "D", "The description of the kit.", GH_ParamAccess.item);
        pManager[0].Optional = true;

    }


    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kits", "K", "Kits from the API.", GH_ParamAccess.list);
    }


    protected override void SolveInstance(IGH_DataAccess DA)
    {
        ApiGoo apiGoo = new();
        var run = false;

        DA.GetData(0, ref apiGoo);
        DA.GetData(1, ref run);

        if (run)
        {
            var kits = apiGoo.Value.GetKits();

            DA.SetDataList(0, kits.Select(k=>new KitGoo(k)));
        }
    }
}