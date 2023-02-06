using System;
using System.Drawing;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructConnectableComponent : ConstructComponent
    {
        public ConstructConnectableComponent() : base("Construct Connectable", "Connectable", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new SobjectParam());
            pManager.AddParameter(new LinkParam());
            pManager[1].Optional = true;
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ConnectableParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            SobjectGoo sobject = new();
            if (!DA.GetData(0, ref sobject)) return;

            LinkGoo link = new ();
            DA.GetData(1, ref link);

            var connectable = new Connectable()
            {
                SobjectId = sobject.Value.Id,
                Link = link.Value
            };

            DA.SetData(0, new ConnectableGoo(connectable));
        }
        public override Guid ComponentGuid => new("C20F4D78-1178-4700-973F-6AB81DAC35F1");
        protected override Bitmap Icon => Resources.icon_construct_connectable;
    }
}