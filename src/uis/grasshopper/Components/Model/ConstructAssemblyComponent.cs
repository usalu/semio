using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructAssemblyComponent : ConstructComponent
    {
        public ConstructAssemblyComponent() : base("Construct Assembly", "Assembly", "", "semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new SobjectParam(),"Sobject", "So", "", GH_ParamAccess.item);
            pManager.AddParameter(new AssemblyParam(), "Parts", "Ps", "", GH_ParamAccess.list);
            pManager[1].Optional=true;
        }
        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new AssemblyParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            SobjectGoo sobject = new();
            if (!DA.GetData(0, ref sobject)) return;

            var children = new List<AssemblyGoo>();
            DA.GetDataList(1, children);

            Assembly assembly = new Assembly()
            {
                SobjectId = sobject.Value.Id,
            };
            assembly.Parts.AddRange(children.Select(x=>x.Value));
            DA.SetData(0, new AssemblyGoo(assembly));
        }
        public override Guid ComponentGuid => new("712F6DA6-F74D-449F-83CE-956255450413");
        protected override Bitmap Icon => Resources.icon_construct_assembly;
    }
}