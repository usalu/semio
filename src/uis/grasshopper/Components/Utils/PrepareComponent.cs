using System;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Utils
{
    public class PrepareComponent : GH_Component
    {
        public PrepareComponent()
          : base("Prepare", "Prepare", "", "semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddGenericParameter("Input","I","",GH_ParamAccess.item);
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddTextParameter("Output","O","",GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            var obj = new GH_ObjectWrapper();
            if (!DA.GetData(0, ref obj)) return;
            if (!DA.GetData(0, ref obj)) return;
            if (obj == null) return;
            dynamic semioObject = ((dynamic)obj.Value).Value;
            DA.SetData(0, Semio.Utils.ToBase64(semioObject));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_prepare;
        public override Guid ComponentGuid => new ("3053B0B4-8A3C-4774-9C7A-046472EF8F8A");

    }
}