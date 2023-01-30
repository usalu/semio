using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper
{
    public class ConstructLayoutComponent : ConstructComponent
    {
        public ConstructLayoutComponent()
          : base("Construct Layout", "Layout", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new SobjectParam(),"Sobjects", "S", "", GH_ParamAccess.list);
            pManager.AddParameter(new ConnectionParam(), "Connections", "C", "", GH_ParamAccess.list);
            pManager[1].Optional = true;
            pManager.AddParameter(new LayoutStrategyParam(), "Layout Strategy", "LS","",GH_ParamAccess.item);
            pManager[2].Optional = true;
            pManager.AddParameter(new AssemblyParam(),"Assembly", "A", "", GH_ParamAccess.list);
            pManager[3].Optional = true;
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new LayoutParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            var sobjects = new List<SobjectGoo>();
            if (!DA.GetDataList(0, sobjects)) return;

            var connections = new List<ConnectionGoo>();
            DA.GetDataList(1, connections);

            LayoutStrategyGoo strategy = new();
            DA.GetData(2, ref strategy);

            var assemblies = new List<AssemblyGoo>();
            DA.GetDataList(3, assemblies);

            Layout layout = new Layout()
            {
                Strategy = strategy.Value
            };

            layout.Sobjects.AddRange(sobjects.Select(x => x.Value));
            layout.Connections.AddRange(connections.Select(x => x.Value));
            layout.Assemblies.AddRange(assemblies.Select(x => x.Value));

            DA.SetData(0, new LayoutGoo(layout));
        }
        public override Guid ComponentGuid => new("E866DD2D-3A02-4540-8A05-F9C0387F7503");
        protected override Bitmap Icon => Resources.icon_construct_layout;
    }
}