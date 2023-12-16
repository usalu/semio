using System;
using System.Drawing;
using System.Linq;
using Grasshopper.Kernel;
using Semio.Grasshopper.Goos;
using Semio.Grasshopper.Params;

namespace Semio.Grasshopper.Components;

public class KitsComponent : SemioComponent
{
    public KitsComponent()
        : base("Kits", "Kits",
            "Get all kits from an API.",
            "semio2", "Kits")
    {
    }


    protected override Bitmap Icon =>
        //You can add image files to your project resources and access them like this:
        // return Resources.IconForThisComponent;
        null;


    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ApiParam(), "API", "A", "API to get kits from.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Run the component.", GH_ParamAccess.item, false);
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