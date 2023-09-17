using System;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Data;
using Grasshopper.Kernel.Types;
using Semio.UI.Grasshopper.Properties;
using Semio.UI.Grasshopper.Utility;

namespace Semio.UI.Grasshopper.Components.Utils
{
    public class SerializeComponent : GH_Component
    {
        public SerializeComponent()
          : base("Serialize", "Serialize", "", "Semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddGenericParameter("Input","I","",GH_ParamAccess.tree);
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddTextParameter("Serialized","S","",GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //if (!DA.GetDataTree(0, out GH_Structure<GH_Goo<dynamic>> tree)) return;
            GH_Goo<IGH_Goo> goo = new();
            if (!DA.GetData(0,ref goo) return;
            //var obj = new GH_ObjectWrapper();

            //if (!DA.GetData(0, ref obj)) return;
            //if (obj == null) return;

            //dynamic input = ((dynamic)obj.Value).Value;
            DA.SetData(0, SpeckleConverter.ToString(tree));
        }
        public override Guid ComponentGuid => new ("537857BE-97F2-4A22-BBED-B0C230BA4D8A");

    }
}