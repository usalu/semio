using System;
using System.Linq;
using Grasshopper.Kernel;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class DeconstructPrototypeComponent : DeconstructComponent
    {
        public DeconstructPrototypeComponent()
          : base("Deconstruct Prototype", "DePrototype", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new PrototypeParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddTextParameter("Plan Hash", "H", "", GH_ParamAccess.item);
            pManager.AddParameter(new RepresentationParam(), "Representations", "R", "", GH_ParamAccess.list);
            pManager.AddTextParameter("Description", "Dc", "", GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            PrototypeGoo prototype = new();
            if (!DA.GetData(0, ref prototype)) return;
            DA.SetData(0, prototype.Value.PlanHash);
            if (prototype.Value.Representations.Any()) DA.SetDataList(1, prototype.Value.Representations.Select(e => new RepresentationGoo(e)));
            DA.SetDataList(2, prototype.Value.Description);
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_prototype;
        public override Guid ComponentGuid => new ("F7D844B8-495D-4A85-A748-4F400DC85183");

    }
}